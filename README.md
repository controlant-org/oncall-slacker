Map current oncall shift to a Slack usergroup.

## Required Env Vars

- `PAGERDUTY_TOKEN`: a read-only token for PagerDuty
- `PAGERDUTY_SCHEDULE_ID`: the ID of the schedule to query
- `SLACK_BOT_TOKEN`: a bot token from a Slack app, permissions necessary: `users:read.email` and `usergroups:write`
- `SLACK_USERGROUP_ID`: the id of the usergroup in Slack
