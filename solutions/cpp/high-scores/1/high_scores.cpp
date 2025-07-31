#include "high_scores.h"

#include <algorithm>

namespace arcade {

    std::vector<int> HighScores::list_scores() {
        // TODO: Return all scores for this session.
        return scores;
    }

    int HighScores::latest_score() {
        // TODO: Return the latest score for this session.
        int lsc = scores.back();
        return lsc;
    }

    int HighScores::personal_best() {
        // TODO: Return the highest score for this session.
        int max = scores.front();
        for(size_t i{1}; i<scores.size(); i++){
            if(max < scores[i])
                max = scores[i];
        }
        return max;
    }

    std::vector<int> HighScores::top_three() {
        // TODO: Return the top 3 scores for this session in descending order.
        std::vector<int> sc{};
        int t1{-1}, t2{-1}, t3{-1};
        for(int &val : scores){
            if(t1 < val){
                t3 = t2;
                t2 = t1;
                t1 = val;
                continue;
            }
            if(t2 < val){
                t3 = t2;
                t2 = val;
                continue;
            }
            if(t3 < val){
                t3 = val;
            }
        }
        if(t1!=-1)
            sc.push_back(t1);
        if(t2!=-1)
            sc.push_back(t2);
        if(t3!=-1)
            sc.push_back(t3);
        return sc;
    }

}  // namespace arcade
