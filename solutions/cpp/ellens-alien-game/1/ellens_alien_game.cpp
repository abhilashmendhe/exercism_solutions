namespace targets {
// TODO: Insert the code for the alien class here
    class Alien{
        public:
            int x_coordinate;
            int y_coordinate;
            Alien(int x, int y) : x_coordinate(x), y_coordinate(y) {}
            int get_health(){
                return this->health;
            }
            bool hit(){
                if(health > -1)
                    this->health -= 1;
                return true;
            }
            bool is_alive(){
                if(this->health <= 0)
                    return false;
                return true;
            }
            bool teleport(int x_new, int y_new){
                this->x_coordinate = x_new;
                this->y_coordinate = y_new;
                return true;
            }
            bool collision_detection(Alien& other){
                if(this->x_coordinate == other.x_coordinate && this->y_coordinate == other.y_coordinate)
                    return true;
                return false;
            }
        private:
            int health{3};
    
    };
}  // namespace targets