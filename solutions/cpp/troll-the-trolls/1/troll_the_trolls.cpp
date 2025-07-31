namespace hellmath {

// TODO: Task 1 - Define an `AccountStatus` enumeration to represent the four
// account types: `troll`, `guest`, `user`, and `mod`.
enum class AccountStatus {
    troll,
    guest,
    user,
    mod,
};
// TODO: Task 1 - Define an `Action` enumeration to represent the three
// permission types: `read`, `write`, and `remove`.
enum class Action{
    read,
    write,
    remove,
};
// TODO: Task 2 - Implement the `display_post` function, that gets two arguments
// of `AccountStatus` and returns a `bool`. The first argument is the status of
// the poster, the second one is the status of the viewer.
bool display_post (AccountStatus a1, AccountStatus a2){
    int na1 = static_cast<int>(a1);
    int na2 = static_cast<int>(a2);
    if(na1 == 0 && na2 == na1)
        return true;
    else if(na1 != 0)
        return true;
    return false;
}
// TODO: Task 3 - Implement the `permission_check` function, that takes an
// `Action` as a first argument and an `AccountStatus` to check against. It
// should return a `bool`.
bool permission_check(Action a1, AccountStatus a2){
    int na1 = static_cast<int>(a1);
    int na2 = static_cast<int>(a2);
    switch (na1)
    {
    case 0:
        return true;
    case 1:
        if(na2!=1)
            return true;
        else
            return false;
    case 2:
        if(na2 != 0 && na2 != 1 && na2 != 2)
            return true;
        else 
            return false;
    default:
        return false;
    }
}

// TODO: Task 4 - Implement the `valid_player_combination` function that
// checks if two players can join the same game. The function has two parameters
// of type `AccountStatus` and returns a `bool`.
bool valid_player_combination(AccountStatus a1, AccountStatus a2){
    int as1 = static_cast<int>(a1);
    int as2 = static_cast<int>(a2);

    switch (as1)
    {
    case 0:
        if(as2==0)
            return true;
        return false;
    case 1:
        return false;
    case 2:
    case 3:
        if(as2==2 || as2==3)
            return true;
        return false;
    default:
        return false;
    }
}
// TODO: Task 5 - Implement the `has_priority` function that takes two
// `AccountStatus` arguments and returns `true`, if and only if the first
// account has a strictly higher priority than the second.
bool has_priority(AccountStatus a1, AccountStatus a2){
    int as1 = static_cast<int>(a1);
    int as2 = static_cast<int>(a2);

    switch (as1)
    {
        case 3:
            if(as1 != as2)
                return true;
            else 
                return false;
        case 2:
            if(as2 < 2)
                return true;
            else 
                return false;
        case 1:
            if(as2 < 1)
                return true;
            else
                return false;
    }
    return false;
}

}  // namespace hellmath