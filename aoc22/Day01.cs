using System.Collections;

namespace aoc22;

public class Day01 : IPuzzle
{
    public string PartOne(string input)
    {
        int maxCalories = 0;  
        int currentCalories = 0;

        foreach (var line in input.Split(Environment.NewLine)) {
            if (string.IsNullOrEmpty(line)) {
                maxCalories = currentCalories > maxCalories ? currentCalories : maxCalories;
                currentCalories = 0;
                continue;
            }

            currentCalories += int.Parse(line);
        }

        return maxCalories.ToString();
    }

    public string PartTwo(string input)
    {
        List<int> maxCalories = new(3);      
        int currentCalories = 0;

        var lines = input.Split(Environment.NewLine);
        for (int i = 0; i < lines.Length; i++) {
            string? line = lines[i];
            
            if (!string.IsNullOrEmpty(line)) {
                currentCalories += int.Parse(line);
            }

            if (string.IsNullOrEmpty(line) || i == lines.Length - 1) {
                if (maxCalories.Count == 3) {
                    maxCalories[0] = maxCalories[0] < currentCalories ? currentCalories : maxCalories[0];
                } else {
                    maxCalories.Add(currentCalories);
                }

                maxCalories.Sort();
                currentCalories = 0;
            }
        }

        return maxCalories.Sum().ToString();
    }
}
