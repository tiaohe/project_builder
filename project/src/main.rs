// use actix_web::{error, get, middleware::Logger, App, HttpServer, Result};
// use derive_more::{Display, Error};
// use log::info;
//
// #[derive(Debug, Display, Error)]
// #[display(fmt = "my error: {}", name)]
// pub struct MyError {
//     name: &'static str,
// }
//
// impl error::ResponseError for MyError {}
//
// #[get("/")]
// async fn index() -> Result<&'static str, MyError> {
//     let err = MyError { name: "test error" };
//     info!("{}", err);
//     Err(err)
// }
//
//
// fn main() {
//
//     let x = vec![1,2,3];
//     let mut y = vec![];
//     for i in  {
//         y.push(i);
//     }
//     println!("{:?}",y)
// }
fn main() {

}
/*struct Solution;
impl Solution {
    pub fn merge_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head?;
        let mut modify = head.next.take();
        let mut next_sum = modify.as_mut();

        while let Some(node) = next_sum {
            let mut sum = 0;

            while node.val != 0 {
                sum += node.val;
                if let Some(next) = node.next.as_mut() {
                    *node = *next.take();
                } else {
                    break;
                }
            }
            modify.as_mut().unwrap().val = sum;
            next_sum = node.next.as_mut();

            modify = modify?.next.take();
        }

        head.next
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}*/

// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }