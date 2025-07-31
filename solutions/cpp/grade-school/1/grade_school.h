#if !defined(GRADE_SCHOOL_H)
#define GRADE_SCHOOL_H

#include<map>
#include<vector>
#include<string>
namespace grade_school {
    class school{
        public:
            school() = default;

            void add(const std::string & s, int grade);
            std::map<int, std::vector<std::string>> roster() const;
            std::vector<std::string> grade(int g) const;
        private:
            std::map<int, std::vector<std::string>> marks;
    };
}  // namespace grade_school

#endif // GRADE_SCHOOL_H