from datetime import datetime
import pyxel

DISPLAY_INTERVAL = 5
DISPLAY_LIFETIME = 100

# Keys
KEY_LEFT = pyxel.KEY_S
KEY_RIGHT = pyxel.KEY_F
KEY_CLOCKWISE = pyxel.KEY_L
KEY_COUNTERCLOCKWISE = pyxel.KEY_J
KEY_SOFT_DROP = pyxel.KEY_D
KEY_HARD_DROP = pyxel.KEY_SPACE
KEY_HOLD = pyxel.KEY_A
KEY_RETRY = pyxel.KEY_R
KEY_EXIT = pyxel.KEY_Q

# Colors
COLOR_EMPTY = 0
COLOR_I_MINO = 1
COLOR_O_MINO = 2
COLOR_S_MINO = 3
COLOR_Z_MINO = 4
COLOR_J_MINO = 5
COLOR_L_MINO = 6
COLOR_T_MINO = 7
COLOR_BLOCK = 8
COLOR_LINE = 9
COLOR_TEXT = 10

# Sizes
WIDTH = 10
HEIGHT = 21
CELL_SIZE = 8
HOLD_CELL_SIZE = 5
PRIMARY_NEXT_CELL_SIZE = 5
SECONDARY_NEXT_CELL_SIZE = 4
BOX_BORDER_WIDTH = 3
FIELD_BORDER_WIDTH = 1
FIELD_WIDTH = CELL_SIZE * WIDTH + FIELD_BORDER_WIDTH * (WIDTH - 1)
FIELD_HEIGHT = CELL_SIZE * HEIGHT + FIELD_BORDER_WIDTH * (HEIGHT - 1)
HOLD_WIDTH = 30
HOLD_HEIGHT = 20
PRIMARY_NEXT_WIDTH = 30
PRIMARY_NEXT_HEIGHT = 20
SECONDARY_NEXT_WIDTH = 24
SECONDARY_NEXT_HEIGHT = 16
HOLD_X = BOX_BORDER_WIDTH
HOLD_Y = BOX_BORDER_WIDTH
FIELD_X = HOLD_X + HOLD_WIDTH + BOX_BORDER_WIDTH
FIELD_Y = -CELL_SIZE // 2
PRIMARY_NEXT_X = FIELD_X + FIELD_WIDTH + BOX_BORDER_WIDTH
PRIMARY_NEXT_Y = BOX_BORDER_WIDTH
WINDOW_WIDTH = HOLD_WIDTH + FIELD_WIDTH + PRIMARY_NEXT_WIDTH + BOX_BORDER_WIDTH * 4
WINDOW_HEIGHT = FIELD_Y + FIELD_HEIGHT + BOX_BORDER_WIDTH


class Viewer:
    def __init__(self):
        self.map = [[0] * WIDTH for _ in range(HEIGHT)]
        self.hold = 0
        self.next = [0] * 5
        pyxel.init(WINDOW_WIDTH, WINDOW_HEIGHT, title="tetris viewer", capture_scale=3, fps=60)
        color_map = pyxel.colors.to_list()
        color_map[0] = 0x111111  # empty
        color_map[1] = 0x00eeee  # I mino
        color_map[2] = 0xeeee00  # O mino
        color_map[3] = 0x00ee00  # S mino
        color_map[4] = 0xee0000  # Z mino
        color_map[5] = 0x2222ee  # J mino
        color_map[6] = 0xee7700  # L mino
        color_map[7] = 0x7700ee  # T mino
        color_map[8] = 0x999999  # block
        color_map[9] = 0x555555  # line
        color_map[10] = 0xeeeeee  # text
        pyxel.colors.from_list(color_map)
        self.pieces = 0
        self.attacks = 0
        self.effect = ""
        self.btb = False
        self.ren = 0
        self.game_timer = datetime.now()
        self.btb_display_counter = 0
        self.ren_display_counter = 0
        self.effect_display_counter = 0
        self.fps_timer = datetime.now()
        self.fps = 0
        self.read()

    def run(self):
        pyxel.run(self.update, self.draw)

    def read(self):
        data = input().split(",")
        for i in range(HEIGHT):
            self.map[i] = [int(d) for d in data[i * WIDTH:(i + 1) * WIDTH]]
        self.hold, *self.next, attacks, btb, ren = [int(d) for d in data[WIDTH * HEIGHT:-1]]
        effect = "\n".join(" " * i + d for i, d in enumerate(data[-1].split()))
        self.attacks += attacks
        if self.btb and attacks > 0 and effect != "" and effect != "Perfect\n Clear":
            self.btb_display_counter = DISPLAY_LIFETIME
        self.btb = btb > 0
        if ren > 1:
            self.ren = ren - 1
            self.ren_display_counter = DISPLAY_LIFETIME
        if effect:
            self.effect = effect
            self.effect_display_counter = DISPLAY_LIFETIME

    def update(self):
        if self.btb_display_counter > 0:
            self.btb_display_counter -= 1
        if self.ren_display_counter > 0:
            self.ren_display_counter -= 1
        else:
            self.ren = 0
        if self.effect_display_counter > 0:
            self.effect_display_counter -= 1
        else:
            self.effect = ""
        # time.sleep(max(0.016 - (datetime.now() - self.timer).total_seconds(), 0))
        if pyxel.frame_count % DISPLAY_INTERVAL == 0:
            self.fps = int(1 / (datetime.now() - self.fps_timer).total_seconds())
        self.fps_timer = datetime.now()
        if pyxel.btnp(KEY_LEFT, hold=10, repeat=2):
            print("Left", flush=True)
            self.read()
        if pyxel.btnp(KEY_RIGHT, hold=10, repeat=2):
            print("Right", flush=True)
            self.read()
        if pyxel.btnp(KEY_CLOCKWISE):
            print("Clockwise", flush=True)
            self.read()
        if pyxel.btnp(KEY_COUNTERCLOCKWISE):
            print("Counterclockwise", flush=True)
            self.read()
        if pyxel.btnp(KEY_SOFT_DROP, hold=2, repeat=2):
            print("SoftDrop", flush=True)
            self.read()
        if pyxel.btnp(KEY_HARD_DROP):
            print("HardDrop", flush=True)
            self.read()
            self.pieces += 1
        if pyxel.btnp(KEY_HOLD):
            print("Hold", flush=True)
            self.read()
        if pyxel.btnp(KEY_RETRY):
            print("Retry", flush=True)
            self.read()
            self.pieces = 0
            self.attacks = 0
            self.ren = 0
            self.game_timer = datetime.now()
        if pyxel.btnp(KEY_EXIT):
            print("Exit", flush=True)

    def draw(self):
        pyxel.cls(COLOR_EMPTY)
        time = (datetime.now() - self.game_timer).total_seconds()
        pyxel.text(2, 40, self.effect, COLOR_TEXT)
        if self.ren > 0:
            pyxel.text(2, 63, f"{self.ren:>3}REN", COLOR_TEXT)
        if self.btb_display_counter:
            pyxel.text(2, 78, f" Back\n  to\n   Back", COLOR_TEXT)
        pyxel.text(2, 120, f"Pcs {self.pieces}", COLOR_TEXT)
        pyxel.text(2, 130, f"PPS {self.pieces / time}", COLOR_TEXT)
        pyxel.text(2, 140, f"Atk {self.attacks}", COLOR_TEXT)
        pyxel.text(2, 150, f"APM {self.attacks / (time / 60)}", COLOR_TEXT)
        pyxel.text(2, 160, f"APP {self.attacks / max(self.pieces, 1)}", COLOR_TEXT)
        pyxel.text(1, 175, f"{self.fps:>4}fps", COLOR_TEXT)
        draw_border(HOLD_X, HOLD_Y, HOLD_WIDTH, HOLD_HEIGHT, COLOR_LINE)
        draw_border(FIELD_X, FIELD_Y, FIELD_WIDTH, FIELD_HEIGHT, COLOR_LINE)
        draw_border(PRIMARY_NEXT_X, PRIMARY_NEXT_Y, PRIMARY_NEXT_WIDTH, PRIMARY_NEXT_HEIGHT, COLOR_LINE)
        for i in range(4):
            draw_secondary_next_box(i)
        for y in range(HEIGHT):
            for x in range(WIDTH):
                mino = self.map[y][x]
                if mino <= 7:
                    draw_field(x, y, mino)
                elif mino <= 14:
                    draw_ghost(x, y, mino - 7)
        draw_mino(HOLD_X, HOLD_Y, HOLD_WIDTH, HOLD_HEIGHT, HOLD_CELL_SIZE, self.hold)
        draw_mino(
            PRIMARY_NEXT_X,
            PRIMARY_NEXT_Y,
            PRIMARY_NEXT_WIDTH,
            PRIMARY_NEXT_HEIGHT,
            PRIMARY_NEXT_CELL_SIZE,
            self.next[0]
        )
        for i in range(4):
            draw_mino(
                PRIMARY_NEXT_X,
                PRIMARY_NEXT_Y + PRIMARY_NEXT_HEIGHT + FIELD_BORDER_WIDTH * (i + 1) + SECONDARY_NEXT_HEIGHT * i,
                SECONDARY_NEXT_WIDTH,
                SECONDARY_NEXT_HEIGHT,
                SECONDARY_NEXT_CELL_SIZE,
                self.next[i + 1]
            )


def draw_border(x, y, w, h, c):
    pyxel.rect(x - BOX_BORDER_WIDTH, y - BOX_BORDER_WIDTH, w + BOX_BORDER_WIDTH * 2, h + BOX_BORDER_WIDTH * 2, c)
    pyxel.rect(x, y, w, h, COLOR_EMPTY)


def draw_secondary_next_box(i):
    x = PRIMARY_NEXT_X
    y = PRIMARY_NEXT_Y + PRIMARY_NEXT_HEIGHT + FIELD_BORDER_WIDTH * (i + 1) + SECONDARY_NEXT_HEIGHT * i
    w = SECONDARY_NEXT_WIDTH
    h = SECONDARY_NEXT_HEIGHT
    border = BOX_BORDER_WIDTH
    border_top = FIELD_BORDER_WIDTH
    pyxel.rect(x - border, y - border_top, w + border * 2, h + border_top + border, COLOR_LINE)
    pyxel.rect(x, y, w, h, COLOR_EMPTY)


def draw_field(x, y, mino):
    x = FIELD_X + (CELL_SIZE + FIELD_BORDER_WIDTH) * x
    y = FIELD_Y + (CELL_SIZE + FIELD_BORDER_WIDTH) * y
    border = FIELD_BORDER_WIDTH
    pyxel.rect(x - border, y - border, CELL_SIZE + border * 2, CELL_SIZE + border * 2, COLOR_LINE)
    pyxel.rect(x, y, CELL_SIZE, CELL_SIZE, mino)


def draw_ghost(x, y, mino):
    x = FIELD_X + (CELL_SIZE + FIELD_BORDER_WIDTH) * x
    y = FIELD_Y + (CELL_SIZE + FIELD_BORDER_WIDTH) * y
    border = FIELD_BORDER_WIDTH
    pyxel.rect(x - border, y - border, CELL_SIZE + border * 2, CELL_SIZE + border * 2, COLOR_LINE)
    pyxel.rect(x, y, CELL_SIZE, CELL_SIZE, mino)
    pyxel.rect(x + border, y + border, CELL_SIZE - border * 2, CELL_SIZE - border * 2, COLOR_EMPTY)


def draw_hold(mino):
    x = HOLD_X + (HOLD_WIDTH - HOLD_CELL_SIZE * 3) // 2
    y = HOLD_Y + (HOLD_HEIGHT - HOLD_CELL_SIZE * 2) // 2
    if mino == 1:
        x = HOLD_X + (HOLD_WIDTH - HOLD_CELL_SIZE * 4) // 2
        y = HOLD_Y + (HOLD_HEIGHT - HOLD_CELL_SIZE) // 2
        pyxel.rect(x, y, HOLD_CELL_SIZE * 4, HOLD_CELL_SIZE, mino)
    elif mino == 2:
        x = HOLD_X + (HOLD_WIDTH - HOLD_CELL_SIZE * 2) // 2
        pyxel.rect(x, y, HOLD_CELL_SIZE * 2, HOLD_CELL_SIZE * 2, mino)
    elif mino == 3:
        pyxel.rect(x + HOLD_CELL_SIZE, y, HOLD_CELL_SIZE * 2, HOLD_CELL_SIZE, mino)
        pyxel.rect(x, y + HOLD_CELL_SIZE, HOLD_CELL_SIZE * 2, HOLD_CELL_SIZE * 1, mino)
    elif mino == 4:
        pyxel.rect(x, y, HOLD_CELL_SIZE * 2, HOLD_CELL_SIZE, mino)
        pyxel.rect(x + HOLD_CELL_SIZE, y + HOLD_CELL_SIZE, HOLD_CELL_SIZE * 2, HOLD_CELL_SIZE * 1, mino)
    elif mino == 5:
        pyxel.rect(x, y, HOLD_CELL_SIZE, HOLD_CELL_SIZE, mino)
        pyxel.rect(x, y + HOLD_CELL_SIZE, HOLD_CELL_SIZE * 3, HOLD_CELL_SIZE * 1, mino)
    elif mino == 6:
        pyxel.rect(x + HOLD_CELL_SIZE * 2, y, HOLD_CELL_SIZE, HOLD_CELL_SIZE, mino)
        pyxel.rect(x, y + HOLD_CELL_SIZE, HOLD_CELL_SIZE * 3, HOLD_CELL_SIZE * 1, mino)
    elif mino == 7:
        pyxel.rect(x + HOLD_CELL_SIZE, y, HOLD_CELL_SIZE, HOLD_CELL_SIZE, mino)
        pyxel.rect(x, y + HOLD_CELL_SIZE, HOLD_CELL_SIZE * 3, HOLD_CELL_SIZE * 1, mino)


def draw_mino(left, top, width, height, size, mino):
    x = left + (width - size * 3) // 2
    y = top + (height - size * 2) // 2
    if mino == 1:
        x = left + (width - size * 4) // 2
        y = top + (height - size) // 2
        pyxel.rect(x, y, size * 4, size, mino)
    elif mino == 2:
        x = left + (width - size * 2) // 2
        pyxel.rect(x, y, size * 2, size * 2, mino)
    elif mino == 3:
        pyxel.rect(x + size, y, size * 2, size, mino)
        pyxel.rect(x, y + size, size * 2, size * 1, mino)
    elif mino == 4:
        pyxel.rect(x, y, size * 2, size, mino)
        pyxel.rect(x + size, y + size, size * 2, size * 1, mino)
    elif mino == 5:
        pyxel.rect(x, y, size, size, mino)
        pyxel.rect(x, y + size, size * 3, size * 1, mino)
    elif mino == 6:
        pyxel.rect(x + size * 2, y, size, size, mino)
        pyxel.rect(x, y + size, size * 3, size * 1, mino)
    elif mino == 7:
        pyxel.rect(x + size, y, size, size, mino)
        pyxel.rect(x, y + size, size * 3, size * 1, mino)


if __name__ == '__main__':
    viewer = Viewer()
    viewer.run()
