# MerlinAI

ひっそりとテトリスAI を作り始めました。

動作確認のため手でテストプレイできるようにしていたら、楽しくなって作り込んでしまいました。

Rust と Python の環境があれば遊べます。


##### 1. リポジトリをクローン

```sh
$ git clone https://github.com/yatabis/MerlinAI.git
$ cd MerlinAI
```

##### 2. 仮想環境を作成

```sh
$ python3 -m venv venv
```

##### 3. 仮想環境のアクティベート

```sh
$ . venv/bin/activate
```

または

```sh
$ venv¥Scripts¥activate
```

##### 4. パッケージのインストール

```sh
(venv)$ pip3 install -r requirements.txt
```

##### 5. プログラムの起動

```sh
(venv)$ cargo run
```

##### 5. 仮想環境の終了

```sh
(venv)$ deactivate
```


##

操作方法については [このあたり](https://github.com/yatabis/MerlinAI/blob/6abf23c9ff21213c45e40457921757750cf841ff/viewer.py#L8-L17) を参照してください。
この定義を書き換えてカスタマイズもできます。

あくまで動作確認用のため、自由落下はありません。
また、ホールドを連続して行うことができます。
