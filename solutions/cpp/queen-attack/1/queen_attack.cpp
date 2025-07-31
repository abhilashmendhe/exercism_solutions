#include "queen_attack.h"

#include<stdexcept>
#include<iostream>
#include<utility>
namespace queen_attack {

    chess_board::chess_board(std::pair<int,int> w, std::pair<int,int> b) : 
        m_w{w}, m_b{b}
    {
        if (w.first <0 || w.first > 7 || w.second < 0 || w.second > 7){
            throw std::domain_error("err");
        }
        if (b.first <0 || b.first > 7 || b.second < 0 || b.second > 7){
            throw std::domain_error("err");
        }
        if (b.first==w.first && b.second==w.second){
            throw std::domain_error("err");
        }
    }
    std::pair<int,int> chess_board::white() const{
        return m_w;
    }
    std::pair<int,int> chess_board::black() const{
        return m_b;
    }
    bool chess_board::can_attack()const {
        std::pair<int,int> temp = m_w;

        while(temp.first >= 0){
            if(temp==m_b){
                return true;
            }
            temp.first--;
        }
        
        temp = m_w;
        while(temp.first < 8){
            if(temp==m_b)
                return true;
            temp.first++;
        }

        temp = m_w;
        while(temp.second >= 0){
            if(temp==m_b)
                return true;
            temp.second--;
        }
        temp = m_w;
        while(temp.second < 8){
            if(temp==m_b)
                return true;
            temp.second++;
        }

        temp = m_w;
        while(temp.first >= 0 && temp.second >= 0){
            if(temp==m_b)
                return true;
            temp.first--;
            temp.second--;
        }

        temp = m_w;
        while(temp.first >= 0 && temp.second < 8){
            if(temp==m_b)
                return true;
            temp.first--;
            temp.second++;
        }

        temp = m_w;
        while(temp.first < 8 && temp.second >= 0){
            if(temp==m_b)
                return true;
            temp.first++;
            temp.second--;
        }

        temp = m_w;
        while(temp.first < 8 && temp.second < 8){
            if(temp==m_b)
                return true;
            temp.first++;
            temp.second++;
        }
        return false;
    }
}  // namespace queen_attack
