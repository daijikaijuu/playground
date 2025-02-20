module Maze = struct
  type cell_type = Floor | Wall (* | Start | Finish *)
  type cell = { mutable cell_type : cell_type }
  type direction = Up | Down | Left | Right

  let neighbor_coords x y = function
    | Up -> (x, y - 1)
    | Down -> (x, y + 1)
    | Left -> (x - 1, y)
    | Right -> (x + 1, y)

  let generate_maze width height =
    let grid : cell array array =
      Array.make_matrix width height { cell_type = Wall }
    in
    let visited = Array.make_matrix width height false in
    let stack = ref [] in
    let start_x = 0 and start_y = 0 in
    visited.(start_x).(start_y) <- true;
    grid.(start_x).(start_y).cell_type <- Floor;
    stack := (start_x, start_y) :: !stack;

    while not (List.is_empty !stack) do
      let x, y = List.hd !stack in
      let neighbors =
        [ Up; Down; Left; Right ]
        |> List.filter_map (fun dir ->
               let nx, ny = neighbor_coords x y dir in
               if
                 nx >= 0 && nx < width && ny >= 0 && ny < height
                 && not visited.(nx).(ny)
               then Some (dir, nx, ny)
               else None)
      in
      match neighbors with
      | [] -> stack := List.tl !stack
      | _ ->
          let idx = Random.int (List.length neighbors) in
          let dir, nx, ny = List.nth neighbors idx in
          let _opposite_dir =
            match dir with
            | Up -> Down
            | Down -> Up
            | Right -> Left
            | Left -> Right
          in
          grid.(x).(y).cell_type <- Floor;
          visited.(nx).(ny) <- true;
          stack := (nx, ny) :: !stack
    done;
    grid

  let print grid =
    let width = Array.length grid in
    let height = if width > 0 then Array.length grid.(0) else 0 in
    for y = 0 to height - 1 do
      for x = 0 to width - 1 do
        print_string
          (match grid.(x).(y).cell_type with Wall -> "██" | Floor -> "  ")
      done;
      print_newline ()
    done
end

let () =
  Random.self_init ();
  let maze = Maze.generate_maze 10 10 in
  Maze.print maze
