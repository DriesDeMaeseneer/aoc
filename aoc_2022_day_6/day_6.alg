BEGIN # AOC DAY 6 ! #
	INT window_size_minus_1 = 13;
	STRING input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
	# THANK YOU CharGPT for the hint about the structure of for loops! #
	FOR i TO UPB input DO
		IF i > window_size_minus_1 THEN 
			BEGIN
				STRING current_window;
				current_window := input[i-window_size_minus_1];
				# print ((current_window, new line)); #
				# now loop over the window#
				# check if char i is in that string, false -> return i #
				# CHAR to_check = input[i]; #
				BOOL found_in_window;
				found_in_window := FALSE;
				FOR j FROM 1 BY 1 TO window_size_minus_1 
				DO
					CHAR to_check = input[i-window_size_minus_1+j];
					FOR k TO UPB current_window 
					DO
						CHAR current_char = current_window[k];
						IF (to_check EQ current_char) THEN
						BEGIN
							found_in_window := TRUE
						END
						FI
					OD;
					current_window +:= to_check 
				OD;
				IF found_in_window EQ FALSE
				THEN
				BEGIN
					print ((i, new line));
					stop
				END
				FI
			END
		FI
	OD
END

