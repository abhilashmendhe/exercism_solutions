#ifndef DOCTOR_DATA_H
#define DOCTOR_DATA_H
#include<string>

namespace star_map {
    enum class System {
        Sol = 1,
        BetaHydri,
        EpsilonEridani,
        AlphaCentauri,
        DeltaEridani,
        Omicron2Eridani
    };
}

namespace heaven{
    class Vessel{
        public:
            std::string name;
            int busters = 0;
            int generation;
            star_map::System current_system;
        Vessel(std::string p_name, int p_num);
        Vessel(std::string p_name, int p_num, star_map::System);
        void make_buster();
        bool shoot_buster();
        heaven::Vessel replicate(std::string p_name);
    };

    std::string get_older_bob(Vessel v1, Vessel v2);

    bool in_the_same_system(Vessel v1, Vessel v2);
}


#endif