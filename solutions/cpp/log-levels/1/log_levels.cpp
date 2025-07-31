#include <string>

namespace log_line {

    std::string message(const char * logs){
        std::string tempstr = logs;
        int index = tempstr.find(':');
        tempstr = tempstr.substr(index+2,std::size(tempstr));
        // std::cout<<tempstr<<std::endl;
        return tempstr;
    }
    std::string log_level(const char * logs){
        std::string tempstr = logs;
        int find = tempstr.find('[');
        int lind = tempstr.find(']');
        tempstr = tempstr.substr(find+1, lind-1);
        return tempstr;
    }
    std::string reformat(const char * logs){
        std::string logLevel = log_level(logs);
        std::string messag  = message(logs);
        return messag + " (" + logLevel + ")";
    }
} // namespace log_line
