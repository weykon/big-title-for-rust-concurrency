use std::thread::scope;

pub fn scope_thread() {
    let num = 32;
    scope(|scope| {
        scope.spawn(|| {
            println!("outside nums is {:?}", num);
        });
    });
    addition_for_scope();
}
// 就是他这个scope保持了这个限定在这里的作用域的情况下确保了num的生命周期
// 也就不用担心是否move

// 然后也可以多个thread在scope里面

fn addition_for_scope() {
    let num = 64;
    scope(|scope| {
        scope.spawn(|| {
            println!("1:: outside num is {:?}", num);
        });
        scope.spawn(|| {
            println!("2:: outside num is {:?}", num);
        });
        scope.spawn(|| {
            println!("3:: outside num is {:?}", num);
        });
    })
}
