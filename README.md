# OVH supply bot

Environment variables: 

- `TELOXIDE_TOKEN`: telegram bot token
- `CHAT_ID`: your chat id with your telegram bot

Usage: `<program name> <url>`

## Cron job example

Run every 10 seconds a search for KS-A supplies

```crontab
TELOXIDE_TOKEN="your-bot-token"
CHAT_ID="bot-chat-id"
# Real supplies link
URL=https://www.ovh.com/engine/apiv6/dedicated/server/datacenter/availabilities/?excludeDatacenters=false&planCode=24ska01-syd&server=24ska01
* * * * * /home/azzen/jobs/ovh-bot $URL >> /home/azzen/jobs/ovh-bot.log 2>&1
* * * * * sleep 10; /<path to user dir>/jobs/ovh-bot $URL >> /<path to user dir>/jobs/ovh-bot.log 2>&1
* * * * * sleep 20; /<path to user dir>/jobs/ovh-bot $URL >> /<path to user dir>/jobs/ovh-bot.log 2>&1
* * * * * sleep 30; /<path to user dir>/jobs/ovh-bot $URL >> /<path to user dir>/jobs/ovh-bot.log 2>&1
* * * * * sleep 40; /<path to user dir>/jobs/ovh-bot $URL >> /<path to user dir>/jobs/ovh-bot.log 2>&1
* * * * * sleep 50; /<path to user dir>/jobs/ovh-bot $URL >> /<path to user dir>/jobs/ovh-bot.log 2>&1
```
