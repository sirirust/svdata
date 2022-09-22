module M1 (input in1_1, in1_2, output out1_1);
endmodule

module M2(input in2_1, output [1:0] out2_1, out2_2);
endmodule

module M3 (input in3_1, in3_2, output out3_1, out3_2);
endmodule

module M4 (input in4_1, in4_2, output [1:0] out4_1, out4_2);
    M1 i1 (.in1_1(in4_1), .in1_2(w3), .out1_1(w1));
    M2 i2 (.in2_1(w1), .out2_1(out4_1), .out2_2(w2));
    M3 i3 (.in3_1(in4_2), .in3_2(w2), .out3_1(out4_2), .out3_2(w3));
endmodule

module M5 (input in5_1, in5_2, output out5_1);
    M1 i1 (.in1_1(in5_1), .in1_2(in5_2), .out1_1(out5_1));
endmodule

module M6 (input [1:0] in6_1, output [1:0] out6_1);
    M4 i4 (.in4_1(in6_1[0]), .in4_2(w1), .out4_1(out6_1), .out4_2(w2));
    M5 i5 (.in5_1(in6_1[1]), .in5_2(w2), .out5_1(w1));
endmodule

module M7 (input [1:0] in7_1, output [1:0] out7_1);
    M6 i6 (.in6_1(in7_1), .out6_1(out7_1));
endmodule