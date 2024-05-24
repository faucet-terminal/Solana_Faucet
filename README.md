## 使用docker启动服务

在github的package中找到当前项目的镜像地址，并使用docker run启动容器。

示例：

```bash
docker run -d -p 8080:8080 --name solana-faucet ghcr.io/acesen/solana_faucet
```

**注意：** ghcr.io/acesen/solana_faucet 为镜像地址，在github package中，该地址可能跟示例地址不同！！！



## API说明

项目启动默认端口： 8080

路径： /claim

参数信息：

| 字段    | 字段说明  | 必填 | 备注                              |
| ------- | --------- | ---- | --------------------------------- |
| rpc     | 测试网RPC | Y    |                                   |
| address | 领水地址  | Y    |                                   |
| amount  | 领水金额  | Y    | 单位是lamport，金额太大链上会失败 |

返回值：

| 字段 | 字段说明 | 必填 | 备注 |
| ---- | -------- | ---- | ---- |
| hash | 交易hash |      |      |

请求示例：

```json
{
    "rpc": "https://api.testnet.solana.com",
    "address": "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
    "amount": 1000000
}
```

返回示例：

```json
{
    "hash": "xxxxxx8G5cL3Rrv4zbCP95tfgnZ1irieZa8k5y8LwkL9YUpzkwXQnLnWXbNNAZL7WsHiWZHzmyY67HWbhkdxxxxxx"
}
```

