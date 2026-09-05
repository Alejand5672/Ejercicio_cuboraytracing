# Ejercicio: cubo por raytracing

Proyecto en Rust que renderiza un cubo mediante raytracing con `raylib`.

## Ramas

| Rama | Contenido |
| --- | --- |
| `main` | Primera versión del ejercicio: cubo con iluminación básica y sin textura. |
| `Cubo_textura` | Versión actual: textura de cuarzo, material con albedo y coeficientes Phong, sombras proyectadas y giro manual. |

Para cambiar entre versiones:

```powershell
git checkout main
# o
git checkout Cubo_textura
```

## Ejecutar el proyecto

Desde la carpeta `ejercicio_cubo`:

```powershell
cargo run
```

La primera ejecución puede tardar porque Cargo descarga y compila las dependencias de Rust y Raylib.

## Controles (rama `Cubo_textura`)

| Tecla | Acción |
| --- | --- |
| Flecha izquierda / derecha | Gira el cubo horizontalmente. |
| Flecha arriba / abajo | Gira el cubo verticalmente. |
| `Esc` o cerrar ventana | Finaliza el programa. |

## Recursos

La textura usada por la rama `Cubo_textura` se encuentra en `assets/ceramica_geometrica.png`. El programa la carga desde CPU, calcula coordenadas UV para cada cara del cubo y consulta el color de cada píxel como albedo. La iluminación incluye luz ambiental, difusa, especular y sombras de la luz puntual sobre el piso.
