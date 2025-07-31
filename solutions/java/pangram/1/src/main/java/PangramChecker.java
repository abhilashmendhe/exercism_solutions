import java.util.HashMap;
import java.util.Map;
public class PangramChecker {

    public boolean isPangram(String input) {
        Map<Character, Integer> m = new HashMap<Character, Integer>();
        
        for(int i=0; i<input.length(); i++) {
        	char c = input.charAt(i);
        	if(c > 64 && c < 91) {
        		c += 32;
        	}
        	if(c > 96 && c < 123)
        		m.put(c, 1);	
        }
        
        
        if(m.size()==26) {
        	return true;
        }
        return false;        
    }

}
