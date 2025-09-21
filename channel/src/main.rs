use std::{thread, time};
// use tokio::sync::broadcast;
/* #[tokio::main]
async */
fn main() {
	// let (tx, mut rx) = broadcast::channel(1);
	let (tx, rx) = std::sync::mpsc::channel();

	// let thread_1 = {
	// 	tokio::spawn(move || {
	// 		println!("THREAD 1: Starting.");
	// 		println!("THREAD 1: Sending of '2'.");
	// 		tx.send(2).unwrap();
	// 		println!("THREAD 1: '2' Sent.");
	//              println!("THREAD 1: Waiting 10 secs.");
	//              tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
	//              println!("THREAD 1: End of waiting.");
	// 		println!("THREAD 1: End.");
	// 	})
	// };
	let thread_1 = {
		std::thread::spawn(move || {
			println!("THREAD 1: Starting.");
			println!("THREAD 1: Sending of '2'.");
			tx.send(2).unwrap();
			println!("THREAD 1: '2' Sent.");
			println!("THREAD 1: Waiting 10 secs.");
			thread::sleep(time::Duration::from_secs(10));
			println!("THREAD 1: End of waiting.");
			println!("THREAD 1: End.");
		})
	};
        
	// let thread_2 = {
	//         tokio::spawn(move || {
	//                 println!("THREAD 2: Starting.");
	// 		println!("THREAD 2: Reception of '2'.");
	//                 println!("THREAD 2: Waiting 5 secs.");
	//                 tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
	//                 println!("THREAD 2: End of waiting.");
	// 		rx.recv().await.unwrap();
	// 		println!("THREAD 2: '2' Recepted.");
	// 		println!("THREAD 2: End.");
	// 	})
	// };
	let thread_2 = {
		std::thread::spawn(move || {
			println!("THREAD 2: Starting.");
			println!("THREAD 2: Reception of '2'.");
			println!("THREAD 2: Waiting 5 secs.");
			thread::sleep(time::Duration::from_secs(5));
			println!("THREAD 2: End of waiting.");
			rx.recv().unwrap();
			println!("THREAD 2: '2' Recepted.");
			println!("THREAD 2: End.");
		})
	};

	// thread_1.await.unwrap();
	// thread_2.await.unwrap();
	thread_1.join().unwrap();
	thread_2.join().unwrap();
}
