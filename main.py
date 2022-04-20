# This is a sample Python script.
import random
import time

import chess
import chess.svg
import sys
from IPython.display import SVG, display
from PyQt5.QtSvg import QSvgWidget
from PyQt5.QtWidgets import QApplication, QWidget, QLabel, QFrame
from matplotlib.backends.backend_qt import MainWindow

PIECE_SYMBOLS = [None, "p", "n", "b", "r", "q", "k"]
PIECE_NAMES = [None, "pawn", "knight", "bishop", "rook", "queen", "king"]

pieces_value = {
    chess.PAWN: 1,
    chess.KNIGHT: 3,
    chess.BISHOP: 3,
    chess.QUEEN: 10,
    chess.ROOK: 5,
    chess.KING: 100
}

moves_analysed = 0


class Cache:
    def __init__(self, max_size):
        self.max_size = max_size
        self.map = dict()

    def add(self, board: chess.Board, value: int):
        if len(self.map.keys()) < self.max_size:
            self.map[board.fen()] = value

    def __contains__(self, board: chess.Board):
        return board.fen() in self.map.keys()

    def __getitem__(self, board: chess.Board):
        return self.map[board.fen()]

    def clear(self):
        self.map.clear()


cache = Cache(1000)


def count_points(board: chess.Board, is_white: bool) -> int:
    """
    Count points based on the sum of all pieces
    :param is_white:
    :param board: the board you want to analyse
    :return: point as int
    """
    final_value = 0
    m = board.piece_map()
    for i in m:
        value = pieces_value[m[i].piece_type]
        if m[i].color == chess.WHITE:
            final_value += value if is_white else -value
        else:
            final_value -= value if is_white else -value
    return final_value


def new_min_max(board: chess.Board, depth: int, is_white: bool, maximising: bool, alpha: int, beta: int):
    global moves_analysed
    global cache
    if depth == 0:
        return count_points(board, is_white), None
    if maximising:
        value = -9999
        m_moves = []
        legal_moves = [x for x in board.legal_moves]
        for move in legal_moves:
            board.push(move)
            v, m = new_min_max(board, depth - 1, is_white, False, alpha, beta)
            if value < v:
                value = v
                m_moves.clear()
                m_moves.append(move)
            elif value == v:
                m_moves.append(move)
            if value >= beta:
                board.pop()
                break
            alpha = min(alpha, value)
            board.pop()
    else:
        value = 9999
        m_moves = []
        for move in board.legal_moves:
            board.push(move)
            v, m = new_min_max(board, depth - 1, is_white, True, alpha, beta)
            if value > v:
                value = v
                m_moves.clear()
                m_moves.append(move)
            elif value == v:
                m_moves.append(move)
            if value <= alpha:
                board.pop()
                break
            beta = max(beta, value)
            board.pop()
    if not m_moves:
        return -1, None
    idx = random.randint(0, len(m_moves) - 1)
    return value, m_moves[idx]


class MainWindow(QWidget):
    def __init__(self):
        super().__init__()

        self.depth = 2
        self.white_turn = True
        self.last_move = None

        self.setGeometry(100, 100, 1100, 1100)

        self.widgetSvg = QSvgWidget(parent=self)
        self.widgetSvg.setGeometry(10, 10, 600, 600)

        self.label = QLabel(self)
        self.label.setFrameStyle(QFrame.Panel | QFrame.Sunken)

        self.chessboard = chess.Board()
        start = time.time()
        v, m = new_min_max(self.chessboard, self.depth, self.white_turn, True, -9999, 9999)
        cache.clear()
        self.white_turn = not self.white_turn
        print("Finding a move took", time.time() - start, "seconds")
        self.chessboard.push(m)
        self.last_move = m
        self.chessboardSvg = chess.svg.board(self.chessboard).encode("UTF-8")
        self.widgetSvg.load(self.chessboardSvg)

    def paintEvent(self, event):
        self.chessboardSvg = chess.svg.board(self.chessboard, lastmove=self.last_move).encode("UTF-8")
        self.widgetSvg.load(self.chessboardSvg)

    def mousePressEvent(self, event):
        print(count_points(self.chessboard, False))
        start = time.time()
        v, m = new_min_max(self.chessboard, self.depth, self.white_turn, True, -9999, 9999)
        cache.clear()
        self.white_turn = not self.white_turn
        print("Finding a move took", time.time() - start, "seconds")
        if not m:
            print("Check mate")
        self.chessboard.push(m)
        self.last_move = m


if __name__ == "__main__":
    app = QApplication([])
    window = MainWindow()
    window.show()
    app.exec()
# Press the green button in the gutter to run the script.
#if __name__ == '__main__':
#    main()

# See PyCharm help at https://www.jetbrains.com/help/pycharm/
