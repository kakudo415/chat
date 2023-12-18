# API

## Channels

```
/channels
```

### Create

```sh
curl -X POST \
-H 'Content-Type: application/json' \
--data '{ "name": "" }' \
localhost:3000/channels
```

```json
{
  "id": "CHANNEL_ID",
  "name": ""
}
```

## Messages

```
/channels/CHANNEL_ID/messages
```

### Send

```sh
curl -X POST \
-H 'Content-Type: application/json' \
--data '{ "text": "" }' \
localhost:3000/channels/CHANNEL_ID/messages
```

```json
{
  "id": "MESSAGE_ID",
  "text": "",
  "channel_id": "CHANNEL_ID"
}
```
