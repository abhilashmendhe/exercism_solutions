#include "allergies.h"

#include<string>
#include<unordered_set>
#include<iostream>
namespace allergies {

    allergy_test::allergy_test(int num) : score(num) {}
    std::unordered_set<std::string> allergy_test::get_allergies() {
        std::unordered_set<std::string> us;
        if(score < 0)
            return us;
        if(score > 255)
            score = score % 256;
        int check = 1;
        while(check <= score){
            int val = (check&score);
            if(val==1)
                us.emplace("eggs");
            if(val==2)
                us.emplace("peanuts");
            if(val==4)
                us.emplace("shellfish");
            if(val==8)
                us.emplace("strawberries");
            if(val==16)
                us.emplace("tomatoes");
            if(val==32)
                us.emplace("chocolate");
            if(val==64)
                us.emplace("pollen");
            if(val==128)
                us.emplace("cats");
                
            check<<=1;
        }
        return us;
    }
    bool allergy_test::is_allergic_to(std::string s) {
        bool flag = false;
        if(!s.compare("eggs")){
            if(1&score)
                flag = true;
        }
        if(!s.compare("peanuts")){
            if(2&score)
                flag = true;
        }
        if(!s.compare("shellfish")){
            if(4&score)
                flag = true;
        }
        if(!s.compare("strawberries")){
            if(8&score)
                flag = true;
        }
        if(!s.compare("tomatoes")){
            if(16&score)
                flag = true;
        }
        if(!s.compare("chocolate")){
            if(32&score)
                flag = true;
        }
        if(!s.compare("pollen")){
            if(64&score)
                flag = true;
        }
        if(!s.compare("cats")){
            if(128&score)
                flag = true;
        }
        return flag;
    }
}  // namespace allergies
