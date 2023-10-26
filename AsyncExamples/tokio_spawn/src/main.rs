const cntr: i8 = 5; 
async fn chant_HareRam() {
    for i in 0..cntr {
        println!("HareRama {i}");
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}

async fn chant_HareKrishana() {
    for i in 0..cntr {
        println!("HareKrishana {i}");
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}

async fn chant_JaiBajrangbali() {
    for i in 0..cntr {
        println!("JaiBajrangBali {i}");
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}

#[tokio::main]
async fn main() {
    let task = tokio::task::spawn(async {
        for i in 0..cntr {
            println!("Om NamahShivaya {i}");
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    });
   // let _ = tokio::join!(
        tokio::spawn(chant_HareRam());
        tokio::spawn(chant_HareKrishana());
        tokio::spawn(chant_JaiBajrangbali());
       // task;
   // );

    // Main task continues executing concurrently with t1 and t2
    for k in 0..5 {
        println!("Shree Ganeshaya Namah {}", k);
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}
