use isahc::{prelude::*, Request};
use std::{env, thread::sleep, time::Duration};

mod pagerduty;
mod slack;

fn required_env(name: &str) -> String {
  env::var(name).expect(&format!("{} is required", name))
}

fn main() {
  let pd_token = required_env("PAGERDUTY_TOKEN");
  let pd_sched = required_env("PAGERDUTY_SCHEDULE_ID");
  let slack_token = required_env("SLACK_BOT_TOKEN");
  let slack_ug = required_env("SLACK_USERGROUP_ID");

  let pd_req = |uri| {
    Request::get(uri)
      .header("Authorization", format!("Token token={}", pd_token))
      .header("Accept", "application/vnd.pagerduty+json;version=2")
      .timeout(Duration::from_secs(5))
      .body(())
      .expect("build request")
      .send()
      .expect("send request")
  };

  let slack_req = |m, uri| {
    Request::builder()
      .method(m)
      .uri(uri)
      .header("Authorization", format!("Bearer {}", slack_token))
      .timeout(Duration::from_secs(5))
      .body(())
      .expect("build request")
      .send()
      .expect("send request")
  };

  loop {
    let sched: pagerduty::Schedule = pd_req(format!("https://api.pagerduty.com/oncalls?schedule_ids[]={}", pd_sched))
      .json()
      .expect("parse schedule");

    let users = sched
      .oncalls
      .into_iter()
      .map(|o| {
        let email = pd_req(o.user.url)
          .json::<pagerduty::User>()
          .expect("parse user")
          .user
          .email;

        slack_req(
          "GET",
          format!("https://slack.com/api/users.lookupByEmail?email={}", email),
        )
        .json::<slack::EmailLookup>()
        .expect("parse email lookup")
        .user
        .id
      })
      .collect::<Vec<_>>()
      .join(",");

    slack_req(
      "POST",
      format!(
        "https://slack.com/api/usergroups.users.update?usergroup={}&users={}",
        slack_ug, users
      ),
    );

    sleep(Duration::from_secs(61));
  }
}
