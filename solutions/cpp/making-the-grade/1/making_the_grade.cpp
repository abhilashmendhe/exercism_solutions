#include <array>
#include <string>
#include <vector>

// Round down all provided student scores.
std::vector<int> round_down_scores(std::vector<double> student_scores) {
    // TODO: Implement round_down_scores
    std::vector<int> round_scores{};
    for(auto sc : student_scores){
        round_scores.push_back(sc);
    }
    return round_scores;
}


// Count the number of failing students out of the group provided.
int count_failed_students(std::vector<int> student_scores) {
    // TODO: Implement count_failed_students
    int count{};
    for(int sc : student_scores){
        if(sc <= 40)
            count++;
    }
    return count;
}

// Determine how many of the provided student scores were 'the best' based on the provided threshold.
std::vector<int> above_threshold(std::vector<int> student_scores, int threshold) {
    // TODO: Implement above_threshold
    std::vector<int> best_scores{};
    for(int sc : student_scores){
        if(sc >= threshold)
            best_scores.push_back(sc);
    }
    return best_scores;
}


// Create a list of grade thresholds based on the provided highest grade.
std::array<int, 4> letter_grades(int highest_score) {
    // TODO: Implement letter_grades
    int nval = (highest_score - 41)/4;
    std::array<int, 4> lg{};
    for(int i=0; i<4; i++) {
        if(highest_score==100)
            lg[i] = 41 + (i * 15);
        else
            lg[i] = 41 + (i * nval);
    }
    return lg;
}

// Organize the student's rank, name, and grade information in ascending order.
std::vector<std::string> student_ranking(std::vector<int> student_scores, std::vector<std::string> student_names) {
    // TODO: Implement student_ranking
    std::vector<std::string> ranks{};
    
    for(int i{}; i<student_scores.size(); i++){
        std::string val = std::to_string(i+1)+". " + student_names[i] + ": " + std::to_string(student_scores[i]);
        ranks.push_back(val);
    }
    return ranks;
}

// Create a string that contains the name of the first student to make a perfect score on the exam.
std::string perfect_score(std::vector<int> student_scores, std::vector<std::string> student_names) {
    // TODO: Implement perfect_score
    std::string name = "";
    for(int i{}; i<student_scores.size(); i++){
        if(student_scores[i] == 100){
            name = student_names[i];
            break;
        }
    }
    return name;
}
