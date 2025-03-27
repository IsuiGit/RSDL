### RSDL Modules

#### RSDL.collider

- `mod`
	- `pub struct Collider {type_, x, y, w, h, vlx, vty, vtx, vby}`
	- `pub enum Direction {Left, Top, Right, Bottom}`
	- `Collider.init(&mut self, vel f32)`
- `collider_move`
	- `Collider.direction_move(&mut self, max [f32; 2], direction Direction)`
- `collider_sys`
	- `Collider.global_collide(&self, max [f32; 2]) -> (bool, bool, bool, bool)`
- `collider_ray`
	- `Collider.ray_cast(&self, object &Colider, direction Direction) -> f32`
- `collider_collision`
	- `Collider.distance_to(&self, object &Collider) -> f32`

#### RSDL.observer

- `mod`
	- `pub struct Observer {playable, objects, window, events}`
- `observer_events`
	- `Observer.proceed_events(&mut self)`
- `observer_sys`
	- `Observer.resize(&mut self, size [f32, 2])`

#### Collider

##### `Collider.init(&mut self, vel: f32)`

Инициализирует скорости перемещения объекта.

Эта функция устанавливает значения скоростей перемещения объекта в четырех направлениях (влево, вверх, вправо и вниз) на основе переданного значения.

###### Параметры

- `vel`: Значение типа `f32`, представляющее скорость перемещения объекта в каждом направлении.

###### Возвращаемое значение

Функция не возвращает значения, но изменяет скорости перемещения объекта.

###### Пример

```rust
let mut collider = Collider { x: 10.0, y: 10.0, w: 5.0, h: 5.0, vlx: 0.0, vty: 0.0, vrx: 0.0, vby: 0.0 };
// Инициализация скоростей перемещения
collider.init(5.0);

println!("Initialized velocities: vlx: {}, vty: {}, vrx: {}, vby: {}", collider.vlx, collider.vty, collider.vrx, collider.vby);
```

###### Примечания

- Все четыре скорости (`vlx`, `vty`, `vrx`, `vby`) будут установлены на одно и то же значение.
- Эта функция может быть полезна для настройки начальных параметров объекта перед его использованием.

##### `Collider.global_collide(&self, max: [f32; 2]) -> (bool, bool, bool, bool)`

Проверяет глобальную коллизию объекта с границами окна.

Эта функция определяет, выходит ли текущий коллайдер за границы заданного окна по четырем направлениям: влево, вверх, вправо и вниз.

###### Параметры

- `max`: Массив из двух элементов типа `f32`, представляющий максимальные размеры окна.
	- `max[0]`: Ширина окна.
	- `max[1]`: Высота окна.

###### Возвращаемое значение

Возвращает кортеж из четырех булевых значений:
- `true`, если объект выходит за левую границу окна (x <= 0.0).
- `true`, если объект выходит за верхнюю границу окна (y <= 0.0).
- `true`, если объект выходит за правую границу окна (x + w >= max[0]).
- `true`, если объект выходит за нижнюю границу окна (y + h >= max[1]).

Если объект не выходит за соответствующую границу, возвращается `false`.

###### Пример

```rust
let collider = Collider { x: -5.0, y: 10.0, w: 50.0, h: 50.0 };
let window_size = [800.0, 600.0];

let collision = collider.global_collide(window_size);
println!("Collision: {:?}", collision); // Вывод: Collision: (true, false, false, false)
```

###### Примечания

- Функция полезна для проверки коллизий с границами окна, чтобы предотвратить выход объектов за пределы видимой области.
- Возвращаемые значения могут быть использованы для принятия решений о том, как обрабатывать столкновения с границами.

##### `Collider.distance_to(&self, object: &Collider) -> f32

Вычисляет расстояние между текущим коллайдером и другим коллайдером.

Эта функция определяет расстояние между границами двух коллайдеров в 2D-пространстве, используя координаты верхнего левого угла и размеры (ширину и высоту) каждого коллайдера.
Если коллайдеры пересекаются, функция возвращает 0.0.
###### Параметры

- `object`: Ссылка на другой коллайдер (`Collider`), с которым необходимо вычислить расстояние.

###### Возвращаемое значение

Возвращает расстояние в виде `f32` между текущим коллайдером и переданным коллайдером.
Если коллайдеры пересекаются, возвращается 0.0.

###### Пример

```rust
let collider_a = Collider { x: 10.0, y: 10.0, w: 5.0, h: 5.0 };
let collider_b = Collider { x: 20.0, y: 15.0, w: 5.0, h: 5.0 };

let distance = collider_a.distance_to(&collider_b);
println!("Расстояние между объектами: {}", distance); // Вывод: Расстояние между объектами: 0.0
```

###### Примечания

- Функция использует простую геометрию для определения расстояния между границами коллайдеров.
- Если коллайдеры находятся в одной плоскости и не пересекаются, функция возвращает положительное значение.
- Если коллайдеры пересекаются, функция возвращает 0.0, что может быть полезно для проверки коллизий.

##### `Collider.ray_cast(&self, object: &Colider, direction: Direction) -> f32`

Выполняет рэйкастинг по направлению движения для определения расстояния до другого коллайдера.

Эта функция вычисляет расстояние от текущего коллайдера до границы другого коллайдера в указанном направлении (влево, вправо, вверх или вниз).

###### Параметры

- `object`: Ссылка на другой коллайдер (`Collider`), до которого необходимо вычислить расстояние.
- `direction`: Направление (`Direction`), в котором будет производиться рэйкастинг.

###### Возвращаемое значение

Возвращает расстояние в виде `f32` до границы другого коллайдера в указанном направлении.

###### Пример

```rust
let collider_a = Collider { x: 10.0, y: 10.0, w: 5.0, h: 5.0 };
let collider_b = Collider { x: 20.0, y: 15.0, w: 5.0, h: 5.0 };

let distance_left = collider_a.ray_cast(&collider_b, Direction::Left);
let distance_right = collider_a.ray_cast(&collider_b, Direction::Right);
let distance_top = collider_a.ray_cast(&collider_b, Direction::Top);
let distance_bottom = collider_a.ray_cast(&collider_b, Direction::Bottom);

println!("Distances: Left: {}, Right: {}, Top: {}, Bottom: {}", distance_left, distance_right, distance_top, distance_bottom);
```

###### Примечания

- Функция возвращает абсолютное значение расстояния, что позволяет использовать её для проверки расстояний независимо от направления.

##### `Collider.direction_move(&mut self, max: [f32; 2], direction: Direction)`

Перемещает объект в указанном направлении с учетом глобальных коллизий.

Эта функция изменяет координаты текущего объекта в зависимости от указанного направления (влево, вверх, вправо или вниз). Перед перемещением проверяется, не выходит ли объект за границы заданного максимального размера.

###### Параметры

- `max`: Массив из двух элементов типа `f32`, представляющий максимальные размеры области, в которой может находиться объект.
- `max[0]`: Ширина области (например, ширина окна).
- `max[1]`: Высота области (например, высота окна).
- `direction`: Направление (`Direction`), в котором будет производиться перемещение.

###### Возвращаемое значение

Функция не возвращает значения, но изменяет координаты объекта в зависимости от направления перемещения.

###### Пример

```rust
let mut collider = Collider { x: 10.0, y: 10.0, w: 5.0, h: 5.0, vlx: 1.0, vty: 1.0, vrx: 1.0, vby: 1.0 };
let window_size = [800.0, 600.0];

// Перемещение влево
collider.direction_move(window_size, Direction::Left);
println!("New position after moving left: ({}, {})", collider.x, collider.y);

// Перемещение вниз
collider.direction_move(window_size, Direction::Bottom);
println!("New position after moving down: ({}, {})", collider.x, collider.y);
```

###### Примечания

- Если объект пытается выйти за границы области, его координаты не изменяются.
- Функция использует метод `global_collide` для проверки коллизий с границами области.

#### Observer

##### `Observer.resize(&mut self, size: [f32, 2])`

Функция скейлинга объектов относительно изменений размера экрана и начальных настроек.

Эта функция изменяет размеры и позиции объектов в зависимости от нового размера окна. Она пересчитывает ширину и высоту каждого объекта, а также их координаты, чтобы сохранить пропорции при изменении размера окна.

###### Параметры

- `size`: Массив из двух элементов типа `f32`, представляющий новый размер окна.
	- `size[0]`: Новая ширина окна.
	- `size[1]`: Новая высота окна.

###### Возвращаемое значение

Функция не возвращает значения, но изменяет размеры и позиции объектов в зависимости от нового размера окна.

###### Пример

```rust
let mut game = Game {
	window: [800.0, 600.0],
	objects: vec![Collider { x: 100.0, y: 100.0, w: 50.0, h: 50.0 }]
};

// Изменение размера окна
game.resize([1024.0, 768.0]);

println!("New window size: {:?}", game.window);
for obj in &game.objects {
	println!("Object position and size: ({}, {}), ({}, {})", obj.x, obj.y, obj.w, obj.h);
}
```

###### Примечания

- Функция использует текущий размер окна (`self.window`) для вычисления коэффициентов масштабирования по осям X и Y (`sc_x` и `sc_y`).
- Все объекты в `self.objects` будут масштабированы, чтобы соответствовать новому размеру окна.
- После выполнения функции новое значение размера окна будет обновлено в `self.window`.

##### `Observer.proceed_events(&mut self)`

Обрабатывает события ввода и управляет движением игрового объекта.

Эта функция проверяет, какие клавиши были нажаты, и в зависимости от этого перемещает играбельный объект (`playable`) в соответствующем направлении. Перед перемещением проверяется возможность движения с учетом коллизий с блокирующими объектами.

###### Параметры

Функция не принимает параметров.

###### Возвращаемое значение

Функция не возвращает значений, но изменяет состояние игрового объекта в зависимости от нажатых клавиш и коллизий с другими объектами.

###### Пример

```rust
let mut game = Game {
	objects: vec![Collider { x: 100.0, y: 100.0, w: 50.0, h: 50.0, type_: COLLIDER_BLOCK }],
	playable: Player { x: 120.0, y: 120.0, vlx: 5.0, vty: 5.0, vrx: 5.0, vby: 5.0 },
	events: vec![SDLK_A] // Пример нажатой клавиши
};

game.proceed_events();
```

###### Примечания

- Функция использует метод `ray_cast` для определения расстояния до блокирующих объектов и метод `distance_to` для проверки пересечения с ними.
- Движение осуществляется только в том случае, если объект не сталкивается с другими коллайдерами в выбранном направлении.
