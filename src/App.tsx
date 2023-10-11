import { useEffect, useState } from "react";
import { Table } from '@douyinfe/semi-ui';

import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

type Data = {
  appName: string
  showName: string
}

function App() {
  const [data, setData] = useState<Data[]>([]);

  const [current, setCurrent] = useState<String>('')
  
  useEffect(() => {
    setInterval(() => {
      console.log('查询');
      invoke("get_focused_app").then(res => {
        setCurrent(res as string)
      })
    },10000)

    invoke("scan_apps").then(res => {
      console.log(res);
      setData((res as string[]).map(appName => {
        return { appName, showName: appName }
      }))
    })
  }, [])

  const columns = [
    {
      title: '应用程序名称',
      dataIndex: 'appName',
    },
    {
      title: '显示名称',
      dataIndex: 'showName',
    },
  ];

  return (
    <Table columns={columns} dataSource={data} pagination={false} />
  );
}

export default App;
