# def hi
#   puts "Hello world!"
# end
require 'set'

total_found_characters = Array.new
group_string_buffer = Array.new

file_name = "day_3.input"

enumeration = 0
File::foreach(file_name) { |line| 
  if not enumeration == 0 and enumeration.modulo(3) == 0 
    # perform all operations needed to find badge.
    # loop over first string
    found_characters = Set.new

    group_string_buffer[0].each_char { |character|
      if group_string_buffer[1].count(character) >=1 and group_string_buffer[2].count(character) >= 1 and not found_characters.include?(character)
        found_characters << character
      end
    }

    group_string_buffer.clear

    for character in found_characters.to_a 
        total_found_characters << character
    end
  end
  group_string_buffer << line
  enumeration += 1
}

total_score = 0

for character in total_found_characters 
  char_encoding = character.ord
  if 64 < char_encoding and char_encoding < 91 
    # puts "Grote letter"
    total_score += char_encoding - 64 + 26
  elsif 96 < char_encoding and char_encoding< 123
    # puts "Kleine letter"
    total_score += char_encoding - 96
  end  
end

puts total_score

