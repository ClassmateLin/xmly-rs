use std::{collections::HashMap};
use serde_json::value::Value;
use reqwest::{header::USER_AGENT, Method};
use chrono::prelude::*;

const M_URL: &str = "http://m.ximalaya.com/web-activity";
const HYBRID_URL: &str = "http://hybrid.ximalaya.com/web-activity";


pub struct XmlyApp {
    token: String,
    nickname: String,
}

impl XmlyApp {

    pub fn new(token: String) -> XmlyApp {
        XmlyApp { token:token, nickname: String::from("未知用户") }
    }

    // 请求数据
    pub async fn request(&mut self, url: &str, body: HashMap<String, Value>, method: Method) -> Result<HashMap<String, Value>, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let res = if method == Method::GET {
            let query = Vec::from_iter(body.iter());
            client.get(url)
                .header(USER_AGENT, "ting_v9.0.36_c5(CFNetwork, iOS 15.4.1, iPhone14,2)")
                .header("cookie", format!("1&_token={}", self.token))
                .query(&query)
                .send()
                .await?
                .json::<HashMap<String, Value>>()
                .await?
        }else{
            client.post(url)
            .header(USER_AGENT, "ting_v9.0.36_c5(CFNetwork, iOS 15.4.1, iPhone14,2)")
            .header("cookie", format!("1&_token={}", self.token))
            .json(&body)
            .send()
            .await?
            .json::<HashMap<String, Value>>()
            .await?
        };
        Ok(res)
    }

    pub async fn refresh_task(&mut self, task_id: String) -> Result<bool, Box<dyn std::error::Error>> {
        let mut map = HashMap::new();
        map.insert("aid".to_string(), Value::from("112"));
        map.insert("taskId".to_string(), Value::from(task_id));
        let data = self.request(&format!("{HYBRID_URL}/task/v2/refreshClientTask"), map, Method::POST).await?;
        
        let err_msg = format!("数据错误: {:?}", data);
        let ret = data.get("ret").expect(&err_msg).to_string();
        if ret != "0" {
            panic!("{}", err_msg);
        }
        let data_ret = data.get("data").expect(&err_msg)    
            .get("ret").expect(&err_msg).to_string();

        let res = data_ret == "0";

        Ok(res)
    }

    //
    pub async fn receive_award(&mut self, task_id: String) -> Result<bool, Box<dyn std::error::Error>> {

        let mut map = HashMap::new();
        map.insert("aid".to_string(), Value::from("112"));
        map.insert("taskId".to_string(), Value::from(task_id));
        let data = self.request(&format!("{M_URL}/task/v2/drawTaskAward"), map, Method::POST).await?;
       
        let err_msg = format!("数据错误: {:?}", data);
        
        let ret = data.get("ret").expect(&err_msg).to_string();
        if ret != "0" {
            panic!("{}", err_msg);
        }
        
        let data_ret = data.get("data").expect(&err_msg).get("status").expect(&err_msg).to_string();

        let res = data_ret == "0";

        Ok(res)
    }

    // 查询信息/签到
    pub async fn sign(&mut self)-> Result<(), Box<dyn std::error::Error>> {
        
        let mut map = HashMap::new();
        map.insert("aid".to_string(), Value::from("87"));
        let data = self.request(&format!("{HYBRID_URL}/signIn/v2/querySignInInfo"), map.clone(), Method::GET).await?;
        let err_msg = format!("查询用户信息失败, {:?}", data);
        let ret = data.get("ret").expect(&err_msg).to_string();
        if ret != "0" {
            panic!("{}", err_msg);
        }

        self.nickname = data.get("context")
            .expect(&err_msg)
            .get("currentUser")
            .expect(&err_msg)
            .get("nickname").expect(&err_msg).to_string();
        
        let user_sign_info = data.get("data").expect(&err_msg)
            .get("signInUserInfo").expect(&err_msg);
        
        let is_today_signed = user_sign_info.get("signInStatus").expect(&err_msg).to_string();
        if is_today_signed == "2" {
            println!("{}, 今日已签到!", self.nickname);
        }else{
            let sign_data = self.request(&format!("{HYBRID_URL}/signIn/v2/signIn"), map, Method::POST).await?;
            println!("{}, 签到结果:{:?}!", self.nickname, sign_data);
        }
        Ok(())
    }

    // 获取任务列表
    pub async fn get_task_list(&mut self) -> Result<Vec<Value>, Box<dyn std::error::Error>> {
        let date = Local::now().format("%Y%m%d").to_string(); 
        let mut params = HashMap::new();
        params.insert("checkData".to_string(), Value::from("JQ+c+CXXlSS1rKGYDJh0tuSI6ciKxG9i1NDMMxocxiScVbJpbvFbkTxQOSjcwsQYDC3N2oJdr6KB+sIlbHGf+yoUC/ysQEE4rVmgA3yTHQdBG7Kf8iasXWk6h+jN4L7UFK3F67hbMCSh+8g9BCB4DsxwxZEs1WqW4wAXhQ1A2x8="));
        params.insert("aid".to_string(), Value::from("112"));
        params.insert("listenTime".to_string(), Value::from("0"));
        params.insert("date".to_string(), Value::from(date));
        let res = self.request(&format!("{M_URL}/task/v2/taskRecords?tag=pc"), params, Method::POST).await?;
        let task_items = res.get("data").expect("error").get("taskItems").expect("获取任务列表失败...").as_array().unwrap();
        Ok(task_items.to_vec())
    }

    /// 任务入口
    pub async fn run(&mut self)-> Result<(), Box<dyn std::error::Error>> {
        self.sign().await?;
        let task_list = self.get_task_list().await?;
        for task in task_list.iter() {
            let task_name = task.get("title").unwrap().to_string();
            let task_type = task.get("taskType").unwrap().to_string();
            let task_id = task.get("id").unwrap().to_string();
            if task_type != "102" {
                continue;
            }
            self.refresh_task("100".to_string()).await?;
            let bool = self.receive_award(task_id.clone()).await?;
            println!("领取任务《{}》奖励, 结果:{}", task_name, bool);
        }
        
        Ok(())
    }
}