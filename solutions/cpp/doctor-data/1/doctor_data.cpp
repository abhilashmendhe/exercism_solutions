#include "doctor_data.h"
#include<string>

heaven::Vessel::Vessel(std::string p_name, int p_num) 
    : name{p_name}, generation{p_num} { 
        current_system = star_map::System::Sol;
}
heaven::Vessel::Vessel(std::string p_name, int p_num, star_map::System s) 
    : name{p_name}, generation{p_num} { 

        current_system = s;
}

void heaven::Vessel::make_buster(){
    busters = 1;
}
bool heaven::Vessel::shoot_buster(){
    bool flag = busters == 0 ? false : true;
    busters--;
    return flag;
}

heaven::Vessel heaven::Vessel::replicate(std::string p_name){
    heaven::Vessel temp{p_name, this->generation+1};
    return temp;
}

std::string heaven::get_older_bob(Vessel v1, Vessel v2){
        return v1.generation < v2.generation ? v1.name : v2.name;
    }

bool heaven::in_the_same_system(Vessel v1, Vessel v2){
    return v1.current_system == v2.current_system ? true : false;
}