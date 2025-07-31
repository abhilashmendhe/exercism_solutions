#if !defined(QUEEN_ATTACK_H)
#define QUEEN_ATTACK_H

#include<utility>
namespace queen_attack {
    class chess_board{

        public:
            chess_board(std::pair<int,int> w, std::pair<int,int> b);
            void print() const;
            std::pair<int,int> white() const;
            std::pair<int,int> black() const;
            bool can_attack()const;
        private:
            std::pair<int,int> m_w;
            std::pair<int,int> m_b;
    };
}  // namespace queen_attack

#endif // QUEEN_ATTACK_H