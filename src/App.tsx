import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { open } from '@tauri-apps/api/shell';
import "./App.css";
import { Button, Spin, Switch, Tooltip, Typography } from "@douyinfe/semi-ui";
import { asyncApp, newActivity, registerDevice } from "./api";
import { timeAgo } from "./utils";
import { server_url } from "./utils/http";

type App = {
  serialNo?: string;
  appName?: string;
  showName?: string;
  appIcon?: string;
  async_at?: string;
};

function App() {
  const [username, setUsername] = useState<string>("");
  const [serialNo, setSerialNo] = useState<string>("");
  const [loading, setLoading] = useState<boolean>(false);
  const [start, setStart] = useState<boolean>(false);
  const [recent, setRecent] = useState<App>({});

  let timer: number = 0;

  useEffect(() => {
    init();
    return () => {
      handleStop();
    };
  }, []);

  function onChangeAsync(checked: boolean) {
    setStart(checked);
    if (checked) {
      handleStart();
    } else {
      handleStop();
    }
  }

  function handleStart() {
    console.log("handleStart");
    if (timer) {
      handleStop();
    }
    timer = setInterval(async () => {
      setLoading(false);
      try {
        const appName: string = await invoke("get_focused_app");
        const data = {
          appName,
          serialNo,
        };
        const res = await newActivity(data);
        const app = JSON.parse(res.data as string).app as App;
        console.log("app", app);
        setRecent(app);
      } catch (e) {
        console.log("get_focused_app error");
        console.log(e);
      }
    }, 5000);
  }

  function handleStop() {
    console.log("handleStop");
    if (timer) {
      clearInterval(timer);
      timer = 0;
    }
  }

  async function init() {
    try {
      setLoading(true);
      const serialNo: string = await invoke("get_serial_number");
      setSerialNo(serialNo);
      const username: string = await invoke("get_user_name");
      setUsername(username);
      await registerDevice({ serialNo, name: username });
      const apps: string[] = await invoke("scan_apps");
      const app_list = apps.map((appName) => {
        return { appName, showName: appName, serialNo };
      });
      await asyncApp(app_list);
      setLoading(false);
    } catch (e) {
      console.log("init error");
      console.log(e);
      setLoading(false);
    }
  }

  return (
    <Spin spinning={loading}>
      <div className="p-4">
        <div>
          <span className="font-bold">{username}</span>
          <Button
            className="text-12px ml-1"
            theme="borderless"
            type="secondary"
          >
            修改
          </Button>
        </div>
        <div className="flex mt-1 items-center">
          <span className="text-12px">开启同步</span>
          <Switch
            onChange={onChangeAsync}
            checked={start}
            className="ml-2"
            size="small"
            aria-label="a switch for demo"
          />
        </div>
        {start ? (
          <>
            <div className="mt-1 text-12px flex items-center">
              <div className="w-12px h-12px bg-green rounded-[50%]"></div>
              <div className="flex items-center ml-2 text-#A7ABB0">
                <div>同步中 </div>
                {recent.showName && (
                  <div className="text-#0064FA mx-2 w-125px truncate truncate-overflow-last-line">
                    使用{recent.showName}
                  </div>
                )}
                {recent.async_at && <div>{timeAgo(recent.async_at)}</div>}
              </div>
            </div>

            {/* <div className="text-12px flex items-center">
              <div className="w-12px h-12px bg-green rounded-[50%]"></div>
              <div className="flex items-center ml-2 text-#A7ABB0">
                <div>
                  正在听
                </div>
                <div className="text-#0064FA mx-2">QQ</div>
                <div>30s前</div>
              </div>
            </div> */}
          </>
        ) : (
          <>
            <div className="mt-1 text-12px flex items-center">
              <div className="w-12px h-12px bg-red rounded-[50%]"></div>
              <div className="flex items-center ml-2 text-#A7ABB0">
                <div>暂未同步</div>
                {recent.showName && (
                  <div className="text-#0064FA mx-2 max-w-125px truncate truncate-overflow-last-line">
                    使用{recent.showName}
                  </div>
                )}
                {recent.async_at && <div>{timeAgo(recent.async_at)}</div>}
              </div>
            </div>
          </>
        )}

        <Tooltip content="点击查看">
          <Typography.Text
            onClick={async () =>
              await open(`${server_url}/activity/${serialNo}`)
            }
          >
            {serialNo}
          </Typography.Text>
        </Tooltip>
      </div>
    </Spin>
  );
}

export default App;
