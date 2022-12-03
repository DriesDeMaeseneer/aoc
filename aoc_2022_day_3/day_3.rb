# def hi
#   puts "Hello world!"
# end
require 'set'

total_found_characters = Array.new
file_name = "day_3.input"

File::foreach(file_name) { |line| 
  search_chars = line[0..(line.size/2)-1]
  find_chars = line[line.size/2..]

  found_characters = Set.new

  search_chars.each_char { |character|
    if find_chars.count(character) >= 1 and not found_characters.include?(character)
      found_characters << character
    end
  }
  for character in found_characters.to_a 
      total_found_characters << character
  end
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

