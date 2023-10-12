import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import { Button, Spin, Switch } from '@douyinfe/semi-ui';

type Data = {
  appName: string;
  showName: string;
};

function App() {
  const [loading, setLoading] = useState<boolean>(false)
  const [data, setData] = useState<Data[]>([]);
  const [current, setCurrent] = useState<string>("");
  const [start, setStart] = useState<boolean>(false)

  useEffect(() => {
    // setInterval(() => {
    //   console.log("查询");
    //   invoke("get_focused_app").then((res) => {
    //     setCurrent(res as string);
    //   });
    // }, 10000);

    invoke("scan_apps").then((res) => {
      console.log(res);
      setData(
        (res as string[]).map((appName) => {
          return { appName, showName: appName };
        })
      );
    });
  }, []);

  function onChangeAsync(checked: boolean) {
    setStart(checked)
  }

  return (
    <Spin spinning={loading}>
      <div className="p-4">
        <div>
          <span className="font-bold">我的MacBook Pro</span>
          <Button className="text-12px ml-2" theme='borderless' type='secondary'>修改</Button>
        </div>
        <div className="flex mt-1 items-center">
          <span className="text-12px">开启同步</span>
          <Switch onChange={onChangeAsync} checked={start} className="ml-2" size="small" aria-label="a switch for demo" />
        </div>
        {
          start ? (
            <>
              <div className="mt-1 text-12px flex items-center">
                <div className="w-12px h-12px bg-green rounded-[50%]"></div>
                <div className="flex items-center ml-2 text-#A7ABB0">
                  <div>
                    同步中 使用
                  </div>
                  <div className="text-#0064FA mx-2">QQ</div>
                  <div>30s前</div>
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
          )
            :
            (
              <>
                <div className="mt-1 text-12px flex items-center">

                  <div className="w-12px h-12px bg-red rounded-[50%]"></div>
                  <div className="flex items-center ml-2 text-#A7ABB0">
                    <div>
                      暂未同步 上次同步
                    </div>
                    <div className="text-#0064FA mx-2">QQ</div>
                    <div>30s前</div>
                  </div>
                </div>
              </>
            )
        }
      </div>
    </Spin>
  );
}

export default App;
