def solve_reactor_paths(puzzle_input: str) -> int:
    graph = {}
    
    # Step 1: Parse the input into a clean adjacency list
    for line in puzzle_input.strip().split('\n'):
        line = line.strip()
        if not line or ':' not in line:
            continue
        node, neighbors_str = line.split(':')
        node = node.strip()
        neighbors = neighbors_str.strip().split()
        graph[node] = neighbors

    # Memoization table to cache states: (node, has_dac, has_fft) -> path_count
    memo = {}

    # Step 2: Define the recursive DFS with condition tracking
    def dfs(node: str, has_dac: bool, has_fft: bool) -> int:
        # Update flags if we encounter the required devices
        if node == 'dac':
            has_dac = True
        if node == 'fft':
            has_fft = True
            
        # Base Case: We hit the reactor output
        if node == 'out':
            return 1 if (has_dac and has_fft) else 0
            
        # Check cache to prevent redundant work
        state = (node, has_dac, has_fft)
        if state in memo:
            return memo[state]
            
        # Accumulate valid paths from all outbound connections
        path_count = 0
        if node in graph:
            for neighbor in graph[node]:
                path_count += dfs(neighbor, has_dac, has_fft)
                
        # Cache and return the result
        memo[state] = path_count
        return path_count

    # Step 3: Initiate pathfinding from the server rack ('svr')
    return dfs('svr', False, False)


# --- Execution ---
if __name__ == "__main__":
    # Load your puzzle input from a local file
    try:
        with open("input.txt", "r") as f:
            user_input = f.read()
        
        result = solve_reactor_paths(user_input)
        print(f"Fewest valid paths visiting both dac and fft: {result}")
        
    except FileNotFoundError:
        print("Please place your puzzle input into a file named 'input.txt' in the same directory.")