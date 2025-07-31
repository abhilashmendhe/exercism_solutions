#if !defined(ALLERGIES_H)
#define ALLERGIES_H

#include<string>
#include<unordered_set>
namespace allergies {
    class allergy_test{
        private:
            int score;
        public:
            allergy_test(int num);
            std::unordered_set<std::string> get_allergies();
            bool is_allergic_to(std::string s);
    };
    
}  // namespace allergies

#endif // ALLERGIES_H