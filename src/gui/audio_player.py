from tkinter import Misc, Frame, Menu, Label, Button, Listbox, Scrollbar, IntVar, StringVar
from tkinter.ttk import Progressbar, Scale
from tkinter.messagebox import askokcancel, showinfo, showerror
from tkinter.filedialog import askopenfilenames


EXTENSION_LIST = [("Все поддерживаемые форматы", ".wav .flac .m4a .mp3 .ogg"),
                  ("Microsoft Wave", ".wav"),
                  ("Free Lossless Audio Codec", ".flac"),
                  ("Apple Lossless", ".m4a"),
                  ("MPEG Layer 3", ".mp3"),
                  ("Ogg Vorbis Audio", ".ogg")]
TITLE_NAME = "Rust | Python | Audio Player"


class AudioPlayer(Frame):
    def __init__(self, master: Misc = None,):
        super().__init__(master)

        self._volume = IntVar(value=80)
        self._song_pos = IntVar(value=50)
        self._status = StringVar(value="Остановлено")

        self._playlist = list[str]()
        self._playlist_variable = StringVar(value=self._playlist)

        # MASTER
        self.master.title(TITLE_NAME)
        self.master.state("zoomed")
        self.master.protocol("WM_DELETE_WINDOW", self._close)

        # MAIN MENU
        menu = Menu(master)
        self.master.config(menu=menu)

        menu.add_command(label="Добавить", command=self._add)
        menu.add_command(label="Справка", command=self._help)

        # BASE FRAMES
        toolbar = Frame(self.master, relief="raised", borderwidth=1)
        playlist = Frame(self.master)
        statusbar = Frame(self.master, relief="raised", borderwidth=1)

        toolbar.pack(side="top", padx=2, pady=2, fill="x")
        playlist.pack(side="top", fill="both", expand=True)
        statusbar.pack(side="bottom", fill="x")

        # TOOLBAR
        play_button = Button(toolbar, text="PLAY", command=self._play)
        stop_button = Button(toolbar, text="STOP", command=self._stop)
        prev_button = Button(toolbar, text="PREV")
        next_button = Button(toolbar, text="NEXT")

        volume_scale = Scale(toolbar, orient="horizontal", length=100, from_=0, to=100, variable=self._volume,
                             command=lambda e: self._volume.set(value=round(float(e))))

        seek_progressbar = Progressbar(toolbar, mode="determinate", length=300, variable=self._song_pos, maximum=100)

        play_button.pack(side="left", padx=2, pady=2)
        stop_button.pack(side="left", padx=2, pady=2)
        prev_button.pack(side="left", padx=2, pady=2)
        next_button.pack(side="left", padx=2, pady=2)
        volume_scale.pack(side="left", padx=2, pady=2)
        seek_progressbar.pack(side="right", padx=2, pady=2)

        # PLAYLIST
        self._playlist_box = Listbox(playlist, listvariable=self._playlist_variable, selectmode="extended")
        playlist_scroll = Scrollbar(playlist, orient="vertical")

        self._playlist_box.pack(side="left", fill="both", expand=True)
        playlist_scroll.pack(side="right", fill="both")

        self._playlist_box.config(yscrollcommand=playlist_scroll.set)
        playlist_scroll.config(command=self._playlist_box.yview)

        # STATUSBAR
        statusbar_label = Label(statusbar, textvariable=self._status)
        statusbar_label.pack(side="left")

    def _close(self):
        if askokcancel("Выход", "Вы действительно хотите выйти?"):
            self.master.destroy()

    def _add(self):
        filepath = askopenfilenames(title="Выберите один или несколько файлов", filetypes=EXTENSION_LIST)

        if filepath != "":
            for file in filepath:
                self._playlist.append(file)
            self._playlist_variable.set(self._playlist)

    @staticmethod
    def _help():
        showinfo(title="Внимание", message="Спасибо за внимание")

    def _play(self):
        selected_indices = self._playlist_box.curselection()

        if len(selected_indices) > 1:
            showerror(title="Ошибка воспроизведения", message="Невозможно воспроизведение нескольких файлов")
            return

        if len(selected_indices) < 1:
            showerror(title="Ошибка воспроизведения", message="Не выбран объект для воспроизведения")
            return

        self.master.title(self._playlist[selected_indices[0]])
        self._status.set("Играет")

    def _stop(self):
        self.master.title(TITLE_NAME)
        self._status.set("Остановлено")


if __name__ == "__main__":
    app = AudioPlayer()
    app.mainloop()
