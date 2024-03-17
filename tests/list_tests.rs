#[cfg(test)]
mod list_test {
    use linked_list::list::List;

#[test]
fn test_push_front() 
{
    // ARRANGE
    let mut my_list = List::build_empty();

    // ACT


    // ASSERT
    assert_eq!(my_list.push_front(32), 1);
    assert_eq!(my_list.push_front(9), 2);
}

#[test]
fn test_peek_front()
{
    // ARRANGE
    let mut my_list = List::build_empty();

    // ACT

    // add elements to the list
    my_list.push_front(32);
    my_list.push_front(43);
    my_list.push_front(-9);

    // ASSERT
    assert_eq!(my_list.peek_front().unwrap(), -9);
    // make list empty and assert again
    my_list = List::build_empty();
    assert_eq!(my_list.peek_front(), None);
}

#[test]
fn test_push_back()
{
    // ARRANGE
    let mut my_list = List::build_empty();
    
    // ACT
    
    // ASSERT
    assert_eq!(my_list.push_back(32), 1);
    assert_eq!(my_list.push_back(2), 2);
    assert_eq!(my_list.push_back(200), 3);
}

#[test]
fn test_peek_back() 
{
    // ARRANGE
    let mut my_list = List::build_empty();

    // ACT
    
    // ASSERT
    // test empty list
    assert_eq!(my_list.peek_back(), None);
    // add elements
    my_list.push_front(90);
    my_list.push_front(4);
    my_list.push_front(1);
    assert_eq!(my_list.peek_back().unwrap(), 90);
    
    my_list.push_back(32);
    assert_eq!(my_list.peek_back().unwrap(), 32);

}

#[test]
fn test_pop_front()
{
    // ARRANGE
    let mut my_list = List::build_empty();

    // ACT
    // add elements to list
    my_list.push_front(32);
    my_list.push_front(2);
    my_list.push_front(3);

    // ASSERT
    assert_eq!(my_list.pop_front().unwrap(), 3);
    assert_eq!(my_list.pop_front().unwrap(), 2);
    assert_eq!(my_list.pop_front().unwrap(), 32);
    assert_eq!(my_list.pop_front(), None);
}

#[test]
fn test_pop_back()
{
    // ARRANGE
    let mut my_list = List::build_empty();

    // ACT
    // add elements to list
    my_list.push_front(32);
    my_list.push_front(2);
    my_list.push_front(3);

    // ASSERT
    assert_eq!(my_list.pop_back().unwrap(), 32);
    assert_eq!(my_list.pop_back().unwrap(), 2);
    assert_eq!(my_list.pop_back().unwrap(), 3);
    assert_eq!(my_list.pop_back(), None);
}
}
