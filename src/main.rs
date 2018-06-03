extern crate LinkedList;

use LinkedList::LkdLt;

fn main() {
    test1();
}

fn test1()
{
    let mut a = LkdLt::new();

    a.add(7);
    a.add(18);
    a.add(27);
    a.add(38);
    a.add(47);
    a.add(58);
    a.add(668);
    a.add(79);

    match a.remove(0) {
        Ok(v) => println!("remove success {:?}",v),
        Err(()) => println!("remove default")
    }

    match a.remove(6) {
        Ok(v) => println!("remove success {:?}",v),
        Err(()) => println!("remove default")
    }

    match a.set(3,10000) {
        Ok(()) => println!("set success"),
        Err(()) => println!("set failed")
    }

    a.add(7);
    a.add(18);
    a.add(27);
    a.add(38);
    a.add(47);
    a.add(58);
    a.add(668);
    a.add(79);


    for i in 0 .. 10 {
        println!("one {:?} ",a.get(i));
    }
    println!("b = {}",a);
    println!("------------------------------------------------------");
    match a.remove(0) {
        Ok(v) => println!("remove success {:?}",v),
        Err(()) => println!("remove default")
    }


    while let Some(n) = a.next(){
        println!("two {}",n);
    }
    println!("------------------------------------------------------");
    while let Some(n) = a.next(){
        println!("three {}",n);
    }
    println!("Hello, world! {} size : {}",a, a.getSize());
}

fn test2()
{
    let b: LkdLt<i32> = LkdLt::new();

    println!("b = {}",b);
}

fn test3()
{
    let mut b: LkdLt<i32> = LkdLt::new();

    b.add(100);
    b.add(200);
    b.add(300);
    b.add(400);
    b.add(500);
    println!("b = {}",b);

    println!("{:?}",b.max());
}