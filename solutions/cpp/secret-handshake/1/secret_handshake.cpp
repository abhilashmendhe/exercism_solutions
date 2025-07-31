#include "secret_handshake.h"
#include<vector>
#include<algorithm>
#include<string>
namespace secret_handshake {
    std::vector<std::string> commands(int n){
        std::vector<std::string> vec{};

        int i{1};
        while(i<=n){

            if(i&n){
                
                switch (i)
                {
                case 1:
                    vec.push_back("wink");
                    break;
                case 2:
                    vec.push_back("double blink");
                    break;
                case 4:
                    vec.push_back("close your eyes");
                    break;
                case 8:
                    vec.push_back("jump");
                    break;
                case 16:
                    std::reverse(vec.begin(), vec.end());
                    break;
                default:
                    break;
                }
            }
            i<<=1;
        }
        return vec;
    }
}  // namespace secret_handshake
