
Fulbito
=======

La idea comenzó con una competencia en la [ECI][1] en la que se pedía programar
la IA de un personaje rellenando un par de metodos vacías. Una de estas es el
_update_, que se llama en cada ciclo para cada jugador, y otra es el
_tenesLaPelota_, que aunque seguramente no recuerde bien el nombre recuerdo que
se llama al momento en el que el jugador obtiene la pelota y tiene una
oportunidad de patear.

A nuestra dosposición se encuentran varios metodos para ver el estado de la
partida, como la posición de un jugador, la de sus compañeros, la ubicación de
los arcos, de la pelota y la posición del equipo rival. Todo esto junto con
variables, atributos de clase y demás, puede ser usado para programar la lógica
de cada jugador.

Una restricción es que todos los jugadores corren a una velocidad fija. En cada
ciclo el jugador puede cambiar la dirección en la que va, y no estoy seguro de
si según las reglas puede detenerse (aunque se podría conseguir el mismo efecto
dando vueltas en el lugar). También, solo puede patear cuando está en posición
de la pelota, y puede decidir la dirección y la intensidad.

Como idea, quiero implementar y entrenar un automata finito no determinista
basado en cadenas de markov para ver que ocurre y a que conclusiones se puede
llegar.



[1]: http://www.dc.uba.ar/events/eci/2018

