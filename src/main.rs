
mod applicative;

use aws_config::Region;
use aws_sdk_athena::{types::{builders::{QueryExecutionContextBuilder, ResultConfigurationBuilder}, QueryExecution, QueryExecutionState}, Client, Config, Error};
// use aws_sdk_athena::model::{StartQueryExecutionRequest, StartQueryExecutionError};

#[::tokio::main]
async fn main() {
  match query_athena().await {
    Ok(_) => println!("Query executed successfully"),
    Err(e) => panic!("Error executing query: {}", e)
  }
}

async fn query_athena() -> Result<(), aws_sdk_athena::Error> {
  let config = Config::builder().region(Region::from_static("ap-southeast-1")).build();
  let ref client = Client::from_conf(config);

  return start_query_execution(client).await.and_then(|query_id| {
    wait_query_complete(client, query_id.as_str()); //TODO: await?
    Ok(())
  }).and_then(|_| {
    // TODO: get query results
    Ok(())
  });
}

fn example_query() -> &'static str {
  "SELECT * FROM table"
}

async fn start_query_execution(client: &aws_sdk_athena::Client) -> Result<String, aws_sdk_athena::Error> {
  let context = QueryExecutionContextBuilder::default()
    .database("tm_data_lake_prod_lake_db")
    .build();
  let result_config = ResultConfigurationBuilder::default()
    .output_location("s3://aws-athena-query-results-1234567890-us-east-1/")
    .build();
  let request = client.start_query_execution()
    .query_string(example_query())
    .query_execution_context(context)
    .result_configuration(result_config)
    .work_group("tm-data-lake-prod-lf_admin");

  let response = request.send().await;
  let query_id = response.and_then(|output| {
    // let query_id = output.query_execution_id.ok_or("No query ID returned")?;
    let query_id = output.query_execution_id.unwrap();
    println!("Query ID: {}", query_id);
    Ok(query_id)
  }).map_err(|err| {
    aws_sdk_athena::Error::from(err)
  });
  return query_id;
}

async fn wait_query_complete(client: &aws_sdk_athena::Client, query_id: &str) {
  let mut is_still_running = true;
  while is_still_running {
    let get_exec = client.get_query_execution()
      .query_execution_id(query_id);
    let response = get_exec.send().await;
    match response {
      Ok(output) => {
        let state = output.query_execution.unwrap().status.unwrap().state.unwrap();
        println!("Query state: {}", state);
        if state == QueryExecutionState::Succeeded {
          is_still_running = false;
          // TODO: sleep
        } else if state == QueryExecutionState::Failed || state == QueryExecutionState::Cancelled {
          panic!("Query failed or was cancelled");
        }
      },
      Err(e) => {
        println!("Error getting query status: {}", e);
        is_still_running = false;
      }
    }
  }
}