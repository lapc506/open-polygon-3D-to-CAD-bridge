PROGRAM main
VAR
  (* Project-local mapping documented in README.md. Verify runtime offsets. *)
  Forward AT %QX0.0 : BOOL;
  Reverse AT %QX0.1 : BOOL;
  Position AT %QW0 : INT;
  Home AT %IX0.0 : BOOL;
  EndStop AT %IX0.1 : BOOL;
  Start AT %IX0.2 : BOOL;
END_VAR

(* Simple interlocked reference axis: never command both directions. *)
IF Start AND NOT EndStop THEN
  Forward := TRUE;
ELSE
  Forward := FALSE;
END_IF;

IF EndStop OR Home THEN
  Reverse := FALSE;
END_IF;

END_PROGRAM
