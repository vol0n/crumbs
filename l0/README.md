Run the server:
```bash
cargo run
```

In other terminal use `curl` to create/get order:

- To get an order by id:
```bash
curl -X GET -d <id> 0.0.0.0:3000/order -i  
```

- To create an order (replace <ORDER> by json):
```bash
curl --header "Content-Type: application/json" \
  --request POST \
  --data <ORDER> \
  http://localhost:3000/order/new
```

Example:
```bash
$ curl --header "Content-Type: application/json" \
  --request POST \
  --data @order \
  http://localhost:3000/order/new
0

$ curl -X GET -d 0 0.0.0.0:3000/order -i
HTTP/1.1 200 OK
content-type: application/json
content-length: 801
date: Fri, 27 Sep 2024 19:11:58 GMT

{"order_uid":"b563feb7b2b84b6test","track_number":"WBILMTESTTRACK","delivery":{"name":"Test Testov","phone":"+9720000000","zip":"2639809","city":"Kiryat Mozkin","address":"Ploshad Mira 15","region":"Kraiot","email":"test@gmail.com"},"payment":{"transaction":"b563feb7b2b84b6test","request_id":"","currency":"USD","amount":1817,"payment_dt":1637907727,"bank":"alpha","delivery_cost":1500,"goods_total":317,"custom_fee":0},"items":[{"chrt_id":9934930,"track_number":"WBILMTESTTRACK","price":453,"rid":"ab4219087a764ae0btest","name":"Mascaras","sale":30,"size":"0","total_price":317,"nm_id":2389212,"brand":"Vivienne Sabo","status":202}],"locale":"en","internal_signature":"","customer_id":"test","delivery_service":"meest","shardkey":"9","sm_id":99,"date_created":"2021-11-26T06:22:19Z","oof_shard":"1"}

$ curl -X GET -d 1 0.0.0.0:3000/order -i
HTTP/1.1 404 Not Found
content-type: application/json
content-length: 4
date: Fri, 27 Sep 2024 19:27:00 GMT

null
```
