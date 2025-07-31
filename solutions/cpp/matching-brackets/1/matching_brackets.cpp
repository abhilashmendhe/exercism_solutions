#include "matching_brackets.h"
#include<iostream>
#include <stack>
#include<string>
namespace matching_brackets {
    bool check(std::string st){
        bool f = true;
        if(st.length()==0)
            return f;

        std::stack<char> s;   
        for(char &ch : st){
            if(ch!='}' && ch!=']' && ch!=')' && ch!='[' && ch!='(' && ch!='{')
                continue;
            if(ch=='{' || ch=='[' || ch=='(') {
                s.push(ch);
                continue;
            }
            if(s.size()==0){
                f = false;
                break;
            }
            char temp = s.top();
            if(ch=='}') { 
                if(temp=='{')
                    s.pop();
                else {
                    f = false;
                    break;
                }
            } else if (ch==']') {
                if(temp=='[')
                    s.pop();
                else {
                    f = false;
                    break;
                }
            } 
            else if(ch==')'){
                if(temp=='(')
                    s.pop();
                else {
                    f = false;
                    break;
                }
            }
        }        
        if(s.size()!=0)
            f = false;
        return f;
    }
}  // namespace matching_brackets
