a couple of notes and learnings from [this](https://youtu.be/WDkv2cKOxx0) YouTube video.

```
fn moveExample() {
    /*
        Will fall over as point initial value
        has moved to point2
    */
    let point = Point::new();
    let point2 = point;

    println!("{:?}",point);
}
```

```
fn mutableMoveExample() {
    /*
      Must define as 'mut'
    */
    let mut point = Point::new();
    point.x = 2.0;

    let point2 = &point;
    let point3 = &point;


    println!("{:?}",point);
    println!("{:?}",point2);
    println!("{:?}",point3);
}
```

```
fn mutableMoveExample() {
    /*
      Will not work as you cannot write
      to point2 as a reference from point.
    */
    let mut point = Point::new();
    point.x = 2.0;

    let point2 = &point;
    point2.x = 2.0;

    let point3 = &point;

    println!("{:?}",point);
    println!("{:?}",point2);
    println!("{:?}",point3);
}
```

```
fn cannotAssignMutableMoreThanOnce() {
  /*
    Will fall over due to limitation on one
    mutable reference at a time. But, won't
    fall over if you don't use the borrow value.
  */

    let mut point1 = Point::new();
    let point2 = &mut point1;
    let point3 = &mut point1;

    println!("{:?}",point2);
}
```
