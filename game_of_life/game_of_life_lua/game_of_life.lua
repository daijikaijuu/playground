-- Constants
local WIDTH = 20
local HEIGHT = 20
local ALIVE = 1
local DEAD = 0

-- Function to initialize the game board with random cells
function initializeBoard()
    local board = {}
    for i = 1, WIDTH do
        board[i] = {}
        for j = 1, HEIGHT do
            board[i][j] = math.random(0, 1)
        end
    end
    return board
end

-- Function to print the current state of the board
function printBoard(board)
    for i = 1, WIDTH do
        for j = 1, HEIGHT do
            io.write(board[i][j] == ALIVE and "*" or " ")
        end
        print()
    end
end

-- Function to count live neighbors for a given cell
function countLiveNeighbors(board, x, y)
    local count = 0
    for i = -1, 1 do
        for j = -1, 1 do
            local neighborX = x + i
            local neighborY = y + j
            if neighborX >= 1 and neighborX <= WIDTH and neighborY >= 1 and neighborY <= HEIGHT then
                count = count + board[neighborX][neighborY]
            end
        end
    end
    count = count - board[x][y] -- Exclude the cell itself
    return count
end

-- Function to update the board based on the Game of Life rules
function updateBoard(board)
    local newBoard = {}
    for i = 1, WIDTH do
        newBoard[i] = {}
        for j = 1, HEIGHT do
            local liveNeighbors = countLiveNeighbors(board, i, j)
            if board[i][j] == ALIVE then
                newBoard[i][j] = (liveNeighbors == 2 or liveNeighbors == 3) and ALIVE or DEAD
            else
                newBoard[i][j] = (liveNeighbors == 3) and ALIVE or DEAD
            end
        end
    end
    return newBoard
end

-- Main function
function main()
    math.randomseed(os.time())
    local currentBoard = initializeBoard()

    for generation = 1, 10 do -- Adjust the number of generations as needed
        print("Generation " .. generation)
        printBoard(currentBoard)
        currentBoard = updateBoard(currentBoard)
    end
end

-- Run the main function
main()
