
class Foo {

    constructor() {
        console.log("Foo");
    }

    bar() {

    }

    baz(x, y, z) {
        console.log(x);
        console.log(y);
        console.log(z);
    }
}

let x = new Foo();
x.bar();
x.baz(1, 2, 3);
