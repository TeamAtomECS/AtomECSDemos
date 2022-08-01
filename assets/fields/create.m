pos = linspace(-200, 1200, 30)
vals = [ 0, 0, 10, 40, 200, 400, 400, 380, 360, 330, 310, 290, 270, 250, 220, 190, 160, 130, 100, 50, 5, 0, -20, -160, -320, -40, 0, 0, 0, 0]


ipos = linspace(-200, 1200, 100);
ivals = interp1(pos, vals, ipos, 'cubic');
plot(ipos, ivals)

% {"extent_spatial": [1.4, 0.1, 0.1], "extent_cells": [40, 40, 15], "position": [0, 0, 0], "grid": [[]]}

field = zeros(numel(ipos), 3);
field(:, 1) = ivals;
s = struct();
s.extent_spatial = [ 1.4, 0.1, 0.1 ];
s.extent_cells = [ numel(ipos), 1, 1 ];
s.position = [ -0.6, 0, 0 ];
s.grid = field * 1e-4;
str = jsonencode(s);