namespace aoc22.tests;

public class Day01Test
{
    private readonly Day01 _day01; 
    private const string PUZZLE_INPUT = """
            1000
            2000
            3000

            4000

            5000
            6000

            7000
            8000
            9000

            10000
            """;

    public Day01Test()
    {
        _day01 = new Day01();
    }

    [Fact]
    public void FirstPart_ShouldBeSuccessful_WhenGivenValidInput()
    {
        // Arrange

        // Act
        var result = _day01.PartOne(PUZZLE_INPUT);

        // Assert
        result.Should().Be("24000");
    }


    [Fact]
    public void SecondPart_ShouldBeSuccessful_WhenGivenValidInput()
    {
        // Arrange

        // Act
        var result = _day01.PartTwo(PUZZLE_INPUT);

        // Assert
        result.Should().Be("45000");
    }
}