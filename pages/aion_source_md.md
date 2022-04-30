# AION Source

<div class="highlight">
  <canvas tabindex="0" data-raw-handle="1" style="width: 1280px; height: 720px; margin-left: auto; margin-right: auto" alt="app" cursor="auto" width="1280" height="720"></canvas>
  <script type="module">import init from './aion_source.js'; init()</script>
</div>

This simulation shows a simulation of the 2D Magneto-Optical trap (MOT) used for the AION project.
At the bottom, an oven supplies a stream of hot strontium atoms, which move upwards into the vacuum chamber.
Large, high-power laser beams are directed into the chamber, which cool the atoms as they interact with these beam and scatter photons.
A narrower 'push' beam then redirects the laser-cooled atoms, which exit the chamber along a horizontal path.

The simulation has been written using AtomECS - [view the source code](https://github.com/TeamAtomECS/AtomECSDemos/blob/master/examples/aion_source.rs).

## Controls

* Click and drag to rotate the example.

## Credit

Elliot Bentine - [@drbentine](https://twitter.com/drbentine)