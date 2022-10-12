# とりあえずメモ
closure で dp をわたすことで"毎回" borrow することになるので borrow checker にひっかからない
逆に dp を渡さない形で closure を定義すると、 closure が定義自身の中に参照を含むので、
closure が使われる全体で単一の borrow を行うことになる？