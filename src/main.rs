use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> io::Result<()> {
    let socket = TcpStream::connect("127.0.0.1:6142").await?;
    let (mut rd, mut wr) = io::split(socket);

    let write_task = tokio::spawn(async move {
        wr.write_all(b"hello\r\n").await?;
        wr.write_all(b"world\r\n").await?;

        // Sometimes, the rust type inferencer needs a little help
        Ok::<_, io::Error>(())
    });

    let mut buf = vec![0; 128];

    loop {
        let n = rd.read(&mut buf).await?;

        // When read() returns Ok(0), it means the stream has been closed
        if n == 0 {
            break;
        }

        println!("GOT {:?}", &buf[..n]);
    }

    Ok(())
}

// memo
// .awaitをまたいで生存するタスクステートは、ヒープに効率よく保存できる型を使うとよい
// 処理Aが.awaitで一時停止したとき、処理Aに関するスタックフレームは消滅するが、必要な変数（状態）はヒープメモリに移動される
// そして再開されるとき、保存された状態からスタックフレームが再構築される。
// この流れでいくと、例えば固定サイズの配列を使うよりもVecを使う方がヒープの相性が良いので、効率的なメモリ管理が可
