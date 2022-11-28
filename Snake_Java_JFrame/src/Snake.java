import javax.swing.JFrame;

public class Snake extends JFrame {
  Display display;
  public Snake() {
    this.add(new Display());
    this.setTitle("Snake");
    this.setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);
    this.pack();
    this.setVisible(true);
    this.setLocationRelativeTo(null);
    
  }
}