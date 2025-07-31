#include "grade_school.h"

#include<map>
#include<vector>
#include<string>
namespace grade_school {
    void school::add(const std::string & s, int grade){
        
        std::vector<std::string> &v = marks[grade];
        
        int size = v.size();
        if(size==0){
            v.push_back(s);
            return;
        }
        v.push_back(s);
        
        while(v[size].compare(v[size-1])<0){
            std::string tmp = v[size];
            v[size] = v[size-1];
            v[size-1] = tmp;
            size--;
        }
    }
    std::map<int, std::vector<std::string>> school::roster()const{
        return marks;
    }
    std::vector<std::string> school::grade(int g) const {
        if(marks.count(g)==0)
            return std::vector<std::string>{};
        return marks.at(g);
    }
}  // namespace grade_school
